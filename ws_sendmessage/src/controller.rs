use crate::models::messenger::Messenger;
use crate::models::user::UserRepository;
use http::HeaderMap;
use lambda_http::aws_lambda_events::apigw::{
    ApiGatewayProxyResponse, ApiGatewayWebsocketProxyRequest,
};
use lambda_http::Body::Text;
use lambda_runtime::{LambdaEvent};

pub struct Controller<M, U>
where
    M: Messenger,
    U: UserRepository,
{
    messenger: M,
    user_repo: U,
}

impl<M: Messenger, U: UserRepository> Controller<M, U> {
    pub fn new(messenger: M, user_repo: U) -> Self {
        Controller {
            messenger,
            user_repo,
        }
    }

    pub async fn handle_event(
        &self,
        event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
    ) -> Result<ApiGatewayProxyResponse, Box<dyn std::error::Error>> {
        self.messenger
            .send(
                &event.payload.body.ok_or("body")?,
                self.user_repo.users_all().await?,
            )
            .await?;

        let mut res_header = HeaderMap::new();
        res_header.insert("Content-Type", "application/json".parse().unwrap());

        Ok(ApiGatewayProxyResponse {
            status_code: 200,
            multi_value_headers: res_header.clone(),
            headers: res_header,
            is_base64_encoded: Some(false),
            body: Some(Text("Success!".into())),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::aws_lambda_events::apigw::{
        ApiGatewayProxyRequest, ApiGatewayProxyRequestContext,
    };
    use lambda_runtime::{Context, LambdaEvent};
    use serde_json::Value;
    use std::collections::HashMap;

    // TODO: implement this test
    #[tokio::test]
    async fn response_is_good_for_simple_input() {
        let id = "ID";

        let mut context = Context::default();
        context.request_id = id.to_string();

        let payload: ApiGatewayProxyRequest<Value> = ApiGatewayProxyRequest {
            resource: None,
            path: None,
            http_method: Default::default(),
            headers: Default::default(),
            multi_value_headers: Default::default(),
            query_string_parameters: Default::default(),
            multi_value_query_string_parameters: Default::default(),
            path_parameters: Default::default(),
            stage_variables: Default::default(),
            request_context: ApiGatewayProxyRequestContext {
                account_id: None,
                resource_id: None,
                operation_name: None,
                stage: None,
                domain_name: None,
                domain_prefix: None,
                request_id: None,
                protocol: None,
                identity: Default::default(),
                resource_path: None,
                path: None,
                authorizer: HashMap::new(),
                http_method: Default::default(),
                request_time: None,
                request_time_epoch: 0,
                apiid: None,
            },
            body: None,
            is_base64_encoded: None,
        };

        let event = LambdaEvent { payload, context };

        // let response = controller(event).await.unwrap();

        // assert_eq!(response.status_code, 200);
    }
}
