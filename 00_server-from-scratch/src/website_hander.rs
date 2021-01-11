use super::http::{Request, Response, StatusCode};
use super::server::Handler;

pub struct WebsiteHandler; 

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response{
        Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string()))
    }
}