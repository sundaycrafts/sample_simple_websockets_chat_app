use crate::models::user::{User, UserRepository};
use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use std::error::Error;

pub struct DynamodbUserRepository {
    dynamo_client: Client,
    table_name: String,
}

impl TryFrom<&HashMap<String, AttributeValue>> for User {
    type Error = Box<dyn Error>;

    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let id: &AttributeValue = value
            .get("connectionId")
            .ok_or("connectionId is not found")?;
        let user = User::new(id.as_s().map_err(|_| "failed to convert to string")?);
        Ok(user)
    }
}

impl DynamodbUserRepository {
    pub fn new(shared_config: &SdkConfig, table_name: &str) -> Self {
        let client = Client::new(&shared_config);
        DynamodbUserRepository {
            dynamo_client: client,
            table_name: table_name.to_string(),
        }
    }
}

#[async_trait]
impl UserRepository for DynamodbUserRepository {
    async fn users_all(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let results = self
            .dynamo_client
            .scan()
            .table_name(&self.table_name)
            .projection_expression("connectionId")
            .send()
            .await?;

        if let Some(items) = results.items {
            let users = items
                .iter()
                .map(|v| -> Result<User, Box<dyn Error>> { User::try_from(v) })
                .collect::<Result<Vec<User>, Box<dyn Error>>>();

            Ok(users?)
        } else {
            Ok(vec![])
        }
    }
}
