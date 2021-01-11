#![allow(dead_code)]
use server::Server;
use website_hander::WebsiteHandler;
// use http::Method;
// use http::Request;

mod http;
mod server;
mod website_hander;

// &self --> a string slice is an immutable reference to a part of a string. 
fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);    
}

