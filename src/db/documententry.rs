use std::time::Duration;

use serde_json::Value;
use tokio::time::Instant;

#[derive(Debug, Clone)]
pub struct DocumentEntry {
    pub bucket: String, // Bucket which the document's in
    pub data: Value, // JSON data
    pub expiry: Option<Instant>, // Expiry time (None => Does not expire)
}

impl DocumentEntry {
    pub async fn new(bucket: &str, data: &Value, ttl: Option<u64>) -> Self {
        Self {
            bucket: bucket.to_string(),
            data: data.clone(),
            expiry: match ttl {
                Some(ttl) => Some(Instant::now() + Duration::from_secs(ttl)),
                None => None
            }
        }
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(expiry) = self.expiry {
            return Instant::now() > expiry;
        }
        false
    }
}