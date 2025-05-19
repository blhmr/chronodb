use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::Instant};
use crate::request::Request;

#[derive(Clone)]
pub struct Database {
    db: Arc<Mutex<HashMap<String, String>>>,
    lifespan: Arc<Mutex<HashMap<String, Instant>>>,
}

impl Database {
    pub async fn new() -> Self {
        Self {
            db: Arc::new(Mutex::new(HashMap::new())),
            lifespan: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub async fn is_expired(&self, key: &str) -> bool {
        let lifespan_map = self.lifespan.lock().await;
        if let Some(&exp_time) = lifespan_map.get(key) {
            return Instant::now() > exp_time;
        }
        false
    }

    pub async fn delete(&self, key: &str) {
        let mut db_map = self.db.lock().await;
        let mut lifespan_map = self.lifespan.lock().await;
        db_map.remove(key);
        lifespan_map.remove(key);
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        if self.is_expired(key).await {
            self.delete(key).await;
            return None;
        }
        let db_map = self.db.lock().await;
        db_map.get(key).cloned()
    }

    pub async fn set(&self, key: String, value: String, ttl: Option<u64>) {
        let mut db_map = self.db.lock().await;
        db_map.insert(key.clone(), value);

        if let Some(sec) = ttl {
            let exp_time = Instant::now() + Duration::from_secs(sec);
            let mut lifespan_map = self.lifespan.lock().await;
            lifespan_map.insert(key, exp_time);
        }
    }

    pub async fn get_all(&self) -> HashMap<String, String> {
        let db_map = self.db.lock().await;
        db_map.clone()
    }

    pub async fn execute(&self, request: Request) -> Result<String, String> {
        match request.method.as_str() {
            "SET" => {
                if request.key.is_none() {
                    return Err("Missing key".to_string());
                }
                if request.value.is_none() {
                    return Err("Missing value".to_string());
                }
                let key = request.key.unwrap();
                let value = request.value.unwrap();
                self.set(key, value, request.ttl).await;
                return Ok("OK".to_string());
            },
            "GET" => {
                if request.key.is_none() {
                    return Err("Missing key".to_string());
                }
                let key = request.key.unwrap();
                if key == "*" {
                    let all_pairs = self.get_all().await;
                    let pairs = serde_json::to_string(&all_pairs);
                    if let Err(e) = pairs {
                        return Err(e.to_string());
                    } else {
                        return Ok(pairs.unwrap());
                    }
                }
                let value = self.get(&key).await;
                if value.is_none() {
                    return Err(format!("No value with key {}", key));
                }
                return Ok(value.unwrap());
            },
            "DELETE" => {
                if request.key.is_none() {
                    return Err("Missing key".to_string());
                }
                let key = request.key.unwrap();
                self.delete(&key).await;
                return Ok("OK".to_string());
            }
            _ => {
                return Err(format!("Unkown method: {}", request.method));
            }
        }
    }
}