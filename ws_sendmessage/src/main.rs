pub mod adapters;
mod controller;
mod models;

use crate::controller::Controller;
use adapters::{ApiGatewayEndpointUrl, ApiGatewayMessenger, DynamodbUserRepository};
use lambda_runtime::{service_fn, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let shared_config = aws_config::from_env().load().await;
    let api_gateway_url = ApiGatewayEndpointUrl {
        api_id: std::env::var("AWS_API_ID").and_then(|s| Ok(s))?,
        region: std::env::var("AWS_REGION").and_then(|s| Ok(s))?,
        stage: std::env::var("AWS_STAGE").and_then(|s| Ok(s))?,
    };
    let messenger = ApiGatewayMessenger::new(&shared_config, api_gateway_url);
    let user_repo = DynamodbUserRepository::new(
        &shared_config,
        &std::env::var("DYNAMO_TABLE_NAME").and_then(|s| Ok(s))?,
    );

    let controller = Controller::new(messenger, user_repo);
    lambda_runtime::run(service_fn(|e| controller.handle_event(e))).await?;
    Ok(())
}
