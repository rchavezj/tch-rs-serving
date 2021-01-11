// use std::net::TcpStream;
use std::io::{Write, Result as IoResult};

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    body: Option<String>,
    status_code: StatusCode
}

impl Response{
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {status_code, body}
    }
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()>{
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream, 
            "HTTP/1.1 {} {}\r\n\r\n{}", 
            self.status_code, 
            self.status_code.reason_phrase(),
            body
        )
    }
}