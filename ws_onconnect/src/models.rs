use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    pub headers: HashMap<String, String>,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: bool,
    #[serde(rename = "multiValueHeaders")]
    pub multi_value_headers: HashMap<String, Vec<String>>,
    #[serde(rename = "requestContext")]
    pub request_context: RequestContext,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestContext {
    #[serde(rename = "apiId")]
    pub api_id: String,
    #[serde(rename = "connectedAt")]
    pub connected_at: u64,
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    #[serde(rename = "domainName")]
    pub domain_name: String,
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "extendedRequestId")]
    pub extended_request_id: String,
    pub identity: Identity,
    #[serde(rename = "messageDirection")]
    pub message_direction: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "requestTime")]
    pub request_time: String,
    #[serde(rename = "requestTimeEpoch")]
    pub request_time_epoch: u64,
    #[serde(rename = "routeKey")]
    pub route_key: String,
    pub stage: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Identity {
    #[serde(rename = "sourceIp")]
    pub source_ip: String,
}
