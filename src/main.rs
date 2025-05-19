mod request;
mod database;
mod handler;

use std::{env, error::Error};
use database::Database;
use tokio::{net::TcpListener};
use handler::handle_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let  args: Vec<String> = env::args().collect();
    let port = args.get(1);
    if port.is_none() {
        return Err("Usage: chronodb <port>".into());
    }
    let listener = TcpListener::bind(format!("localhost:{}", port.unwrap())).await?;
    let database = Database::new().await;
    println!("[+] Database instance created!");
    println!("[+] Server is running on port {} ...", port.unwrap());
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
