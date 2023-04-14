use crate::models::user::User;
use crate::models::messenger::Messenger;
use async_trait::async_trait;
use aws_config::SdkConfig;

use aws_sdk_apigatewaymanagement::primitives::Blob;
use aws_sdk_apigatewaymanagement::{config, Client};
use std::error::Error;

pub struct ApiGatewayEndpointUrl {
    pub api_id: String,
    pub region: String,
    pub stage: String,
}

impl Into<String> for ApiGatewayEndpointUrl {
    fn into(self) -> String {
        format!(
            "https://{api_id}.execute-api.{region}.amazonaws.com/{stage}",
            api_id = self.api_id,
            region = self.region,
            stage = self.stage
        )
    }
}

pub struct ApiGatewayMessenger {
    client: Client,
}

impl ApiGatewayMessenger {
    pub fn new(shared_config: &SdkConfig, url: ApiGatewayEndpointUrl) -> Self {
        let am_conf = config::Builder::from(shared_config)
            .endpoint_url(url)
            .build();

        ApiGatewayMessenger {
            client: Client::from_conf(am_conf),
        }
    }
}

#[async_trait]
impl Messenger for ApiGatewayMessenger {
    async fn send(&self, msg: &str, users: Vec<User>) -> Result<(), Box<dyn Error>> {
        for user in users {
            self.client
                .post_to_connection()
                .connection_id(user.connection_id)
                .data(Blob::new(msg))
                .send()
                .await?;
        }

        Ok(())
    }
}
