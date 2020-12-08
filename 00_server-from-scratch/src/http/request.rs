use super::method::Method;
        
pub struct Request {
    path: String,
    method: Method,
    query_string: Option<String>,
}