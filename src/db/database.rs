use std::{collections::VecDeque, sync::Arc};
use serde_json::Value;
use tokio::sync::Mutex;
use super::documententry::DocumentEntry;

// Each database contains buckets, a bucket is an array of JSON objects

#[derive(Debug, Clone)]
pub struct Database(Arc<Mutex<VecDeque<DocumentEntry>>>);

impl Database {
    pub async fn new() -> Self {
        Self(Arc::new(Mutex::new(VecDeque::new())))
    }

    pub async fn insert(&self, bucket: &str, json_data: &Value, ttl: Option<u64>) {
        let mut db_map = self.0.lock().await;
        db_map.push_front(DocumentEntry::new(bucket, json_data, ttl).await);
    }

    pub async fn get_all_buckets(&self) -> VecDeque<String> {
        let db_map = self.0.lock().await;
        let mut result = VecDeque::new();
        for document in db_map.iter() {
            if !result.contains(&document.bucket) {
                result.push_back(document.bucket.to_string());
            }
        }
        result
    }

    pub async fn get_bucket(&self, bucket: &str) -> VecDeque<DocumentEntry> {
        let mut db_map = self.0.lock().await;
        db_map.retain(|document| document.bucket != bucket || !document.is_expired());
        db_map.iter()
            .filter(|&document| document.bucket == bucket)
            .cloned()
            .collect()
    }
 
    pub async fn get(&self, bucket: &str, pattern: &Value) -> VecDeque<DocumentEntry> {
        let mut db_map = self.0.lock().await;
        db_map.retain(|doc| !doc.is_expired());
        db_map.iter()
            .filter(|doc| {
                (bucket == "*" || doc.bucket == bucket) &&
                Database::matches_pattern(&doc.data, pattern)
            })
            .cloned()
            .collect()
    }

    fn matches_pattern(data: &Value, pattern: &Value) -> bool {
        match (data, pattern) {
            (Value::Object(data_map), Value::Object(pattern_map)) => {
                pattern_map.iter().all(|(k, v)| {
                    data_map.get(k).map_or(false, |dv| Database::matches_pattern(dv, v))
                })
            }
            (Value::Array(data_arr), Value::Array(pattern_arr)) => {
                pattern_arr.iter().all(|p| data_arr.iter().any(|d| Database::matches_pattern(d, p)))
            }
            _ => data == pattern,
        }
    }

    pub async fn delete_bucket(&self, bucket: &str) {
        let mut db_map = self.0.lock().await;
        db_map.retain(|doc| doc.bucket != bucket);
    }

    pub async fn delete(&self, bucket: &str, pattern: &Value) {
        let mut db_map = self.0.lock().await;
        db_map.retain(|document| {
            if document.is_expired() { return false };
            let bucket_matches = bucket == "*" || document.bucket == bucket;
            let pattern_matches = Database::matches_pattern(&document.data, &pattern);
            !(bucket_matches && pattern_matches)
        });
    }
}