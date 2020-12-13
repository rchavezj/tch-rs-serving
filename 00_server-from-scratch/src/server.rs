use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self { Self { addr } }

    pub fn run (self) { 
        println!("Listening on {}", self.addr); 
        let listener = TcpListener::bind(&self.addr); 
    }
}