use super::http::{Request, Response, StatusCode, Method};
use super::server::Handler;

pub struct WebsiteHandler; 

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response{
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>HELLO WORLD :D </h1>".to_string())),
                "/yolo" => Response::new(StatusCode::Ok, Some("<h1>YOLO :D </h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, None)
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}