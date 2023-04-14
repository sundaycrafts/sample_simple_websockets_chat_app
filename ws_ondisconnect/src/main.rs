use aws_common::lambda::ConnectionId;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_http::aws_lambda_events::apigw::{
    ApiGatewayProxyResponse, ApiGatewayWebsocketProxyRequest,
};
use lambda_runtime::{service_fn, Error, LambdaEvent};

async fn handle(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Box<dyn std::error::Error>> {
    let shared_config = aws_config::load_from_env().await;

    let cid: (String, String) =
        ConnectionId::from(event.payload.request_context.connection_id).try_into()?;

    let client = Client::new(&shared_config);

    client
        .delete_item()
        .table_name("websocket-demo")
        .key(cid.0, AttributeValue::S(cid.1))
        .send()
        .await?;

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some("Disconnected.".into()),
        is_base64_encoded: None,
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // https://github.com/aws-samples/simple-websockets-chat-app/blob/master/ondisconnect/app.js
    lambda_runtime::run(service_fn(handle)).await?;
    Ok(())
}
