#![allow(dead_code)]
use server::Server;
use website_hander::WebsiteHandler;
use std::env;

mod http;
mod server;
mod website_hander;

// &self --> a string slice is an immutable reference to a part of a string. 
fn main() {
    let default_path = format!(
        "{}/public", 
        env!("CARGO_MANIFEST_DIR")
    );
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public_path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));    
}

