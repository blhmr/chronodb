use std::error::Error;
use serde_json::{json, Value};
use tokio::{fs::OpenOptions, io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use crate::db::database::Database;

pub async fn handle_connection(
    mut conn: TcpStream,
    database: &Database,
    file_path: &String
) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![0u8; 4096];
    let n = conn.read(&mut buffer).await?;
    if n == 0 { return Ok(()); }
    let request_str = String::from_utf8_lossy(&buffer[..n]);
    println!("[+] Received request: {}", request_str.trim_ascii());
    let request_json: Value = match serde_json::from_str(&request_str) {
        Ok(val) => val,
        Err(e) => {
            let err_response = json!({"status": "ERROR", "reason": "Invalid JSON"});
            conn.write_all(err_response.to_string().as_bytes()).await?;
            return Err(Box::new(e));
        }
    };
    let response = handle_request(database, &request_json, file_path).await;
    let response_str = response.to_string() + "\n";
    conn.write_all(response_str.as_bytes()).await?;
    Ok(())
}

async fn append_to_file(path: &String, content: String) -> tokio::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

async fn handle_request(database: &Database, req: &Value, file_path: &String) -> Value {
    let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let fallback = json!({});

    match method {
        "insert" => {
            let bucket = req.get("bucket").and_then(|b| b.as_str()).unwrap_or("default");
            let data = req.get("data").unwrap_or(&fallback);
            let ttl = req.get("ttl").and_then(|t| t.as_u64());
            database.insert(bucket, data, ttl).await;
            if file_path != "=" {
                let content = json!({
                    "method": method,
                    "bucket": bucket,
                    "data": data,
                });
                let content = serde_json::to_string::<Value>(&content).unwrap() + "\n";
                // fs::io::
                if let Err(e) = append_to_file(file_path, content.clone()).await {
                    eprintln!("[X] Could not write {} to {}: {}", content, file_path, e);
                }
            }
            json!({ "status": "OK" })
        }

        "get" => {
            let pattern = req.get("pattern").unwrap_or(&fallback);
            let bucket = req.get("bucket")
                .and_then(|b| b.as_str())
                .unwrap_or("*");
            let results = database.get(bucket, pattern).await;
            let json_results: Vec<Value> = results.into_iter().map(|doc| doc.data).collect();
            if file_path != "=" {
                let content = json!({
                    "method": method,
                    "bucket": bucket,
                    "pattern": pattern
                });
                let content = serde_json::to_string::<Value>(&content).unwrap() + "\n";
                if let Err(e) = append_to_file(file_path, content.clone()).await {
                    eprintln!("[X] Could not write {} to {}: {}", content, file_path, e);
                }
            }
            json!({ "status": "OK", "data": json_results })
        }

        "get_all_buckets" => {
            let buckets = database.get_all_buckets().await;
            if file_path != "=" {
                let content = json!({
                    "method": method,
                });
                let content = serde_json::to_string::<Value>(&content).unwrap() + "\n";
                if let Err(e) = append_to_file(file_path, content.clone()).await {
                    eprintln!("[X] Could not write {} to {}: {}", content, file_path, e);
                }
            }
            json!({ "status": "OK", "data": buckets })
        }

        "get_bucket" => {
            let bucket = req.get("bucket").and_then(|b| b.as_str()).unwrap_or("default");
            let results = database.get_bucket(bucket).await;
            let json_results: Vec<Value> = results.into_iter().map(|doc| doc.data).collect();
            if file_path != "=" {
                let content = json!({
                    "method": method,
                    "bucket": bucket
                });
                let content = serde_json::to_string::<Value>(&content).unwrap() + "\n";
                if let Err(e) = append_to_file(file_path, content.clone()).await {
                    eprintln!("[X] Could not write {} to {}: {}", content, file_path, e);
                }
            }
            json!({ "status": "OK", "data": json_results })
        }

        "get_all" => {
            let buckets = database.get_all_buckets().await;
            let mut result_map = serde_json::Map::new();
            for bucket in buckets {
                let docs = database.get_bucket(&bucket).await;
                let values: Vec<Value> = docs.into_iter().map(|doc| doc.data).collect();
                result_map.insert(bucket, json!(values));
            }
            if file_path != "=" {
                let content = json!({
                    "method": method
                });
                let content = serde_json::to_string::<Value>(&content).unwrap() + "\n";
                if let Err(e) = append_to_file(file_path, content.clone()).await {
                    eprintln!("[X] Could not write {} to {}: {}", content, file_path, e);
                }
            }
            json!({ "status": "OK", "data": Value::Object(result_map) })
        }

        "delete" => {
            let pattern = req.get("pattern").unwrap_or(&fallback);
            let bucket = req.get("bucket")
                .and_then(|b| b.as_str())
                .unwrap_or("*");
            database.delete(bucket, pattern).await;
            if file_path != "=" {
                let content = json!({
                    "method": method,
                    "bucket": bucket,
                    "pattern": pattern
                });
                let content = serde_json::to_string::<Value>(&content).unwrap() + "\n";
                if let Err(e) = append_to_file(file_path, content.clone()).await {
                    eprintln!("[X] Could not write {} to {}: {}", content, file_path, e);
                }
            }
            json!({ "status": "OK" })
        }

        "delete_bucket" => {
            let bucket = req.get("bucket").and_then(|b| b.as_str()).unwrap_or("default");
            database.delete_bucket(bucket).await;
            if file_path != "=" {
                let content = json!({
                    "method": method,
                    "bucket": bucket
                });
                let content = serde_json::to_string::<Value>(&content).unwrap() + "\n";
                if let Err(e) = append_to_file(file_path, content.clone()).await {
                    eprintln!("[X] Could not write {} to {}: {}", content, file_path, e);
                }
            }
            json!({ "status": "OK" })
        }

        _ => json!({ "status": "ERROR", "message": "Unknown method" }),
    }
}
