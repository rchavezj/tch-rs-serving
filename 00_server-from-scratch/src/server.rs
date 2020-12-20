use std::io::Read;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::http::Request;

use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self { Self { addr } }

    pub fn run (self) { 
        println!("Listening on {}", self.addr); 
        let listener = TcpListener::bind(&self.addr).unwrap(); 

        loop {
            match listener.accept() {
                Ok((mut stream, _ )) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer)  {
                        Ok(_) => {
                            println!(
                                "Recieved a request: {}", 
                                String::from_utf8_lossy(&buffer)
                            );
                            match Request::try_from(&buffer[..]) {
                                Ok (request) => {},
                                Err(e) => println!("Failed to parse a request: {}", e)
                            }
                            let res: &Result<Request, _> = &buffer[..].try_into();

                        }
                        Err(e) => println!("Failed to establish a connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}