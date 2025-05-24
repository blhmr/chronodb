mod db;
mod server;

use db::database::Database;
use server::handler::handle_connection;
use tokio::net::TcpListener;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let port = args.get(1).map(|s| s.as_str()).unwrap_or("8080");
    let addr = format!("127.0.0.1:{}", port);
    let database = Database::new().await;
    let listener = TcpListener::bind(&addr).await?;
    println!("[+] Server running on {}", addr);
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("[+] New connection from: {}", addr);
        let db = database.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, &db).await {
                eprintln!("Connection error: {}", e.to_string());
            }
        });
    }
}
