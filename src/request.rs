use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub method: String,
    pub key: Option<String>,
    pub value: Option<String>,
    pub ttl: Option<u64>,
}

impl Request {
    pub async fn from_string(request_json: &str) -> Result<Request, serde_json::Error> {
        serde_json::from_str::<Request>(request_json)
    }
}