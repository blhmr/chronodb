use std::error::Error;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use crate::{database::Database, request::Request};

pub async fn handle_connection(mut conn: TcpStream, database: &Database) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0u8; 2048];
    let n = conn.read(&mut buffer).await?;
    if n == 0 { return Ok(()) }
    let request_string = String::from_utf8_lossy(&buffer[..n]);
    let request_string = request_string.trim_ascii();
    println!("[+] Received request: {}", request_string);
    let request_parsed = Request::from_string(&request_string).await;
    if let Err(ref e) = request_parsed {
        conn.write(e.to_string().as_bytes()).await?;
    }
    let response = database.execute(request_parsed.unwrap()).await;
    if let Err(e) = response {
        println!("[X] Sent error: {}", e);
        conn.write(e.as_bytes()).await?;
    } else {
        let response = response.unwrap();
        println!("[+] Sent response: {}", response);
        conn.write(response.as_bytes()).await?;
    }
    Ok(())
}