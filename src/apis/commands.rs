use std::fmt::Display;

use serde_derive::Deserialize;

use super::ApiResponse;

#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(rename = "body")]
    pub body: serde_json::Value,

    #[serde(rename = "message")]
    pub message: String,

    #[serde(rename = "statusCode")]
    pub status_code: u32,
}

impl ApiResponse for Response {}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = &self.body;
        write!(f, "{}", body)
    }
}
