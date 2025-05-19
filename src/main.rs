mod request;
mod database;
mod handler;

use std::{error::Error};
use database::Database;
use tokio::{net::TcpListener};
use handler::handle_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("localhost:8080").await?;
    let database = Database::new().await;
    loop {
        let (conn, addr) = listener.accept().await?;
        println!("[+] New connection from {}", addr);
        let database = database.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(conn, &database).await {
                eprintln!("[X] Error handling connection: {}", e.to_string());
            }
        });
    }
}
