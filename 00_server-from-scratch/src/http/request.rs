use super::method::Method;
use std::convert::TryFrom;
        
pub struct Request {
    path: String,
    method: Method,
    query_string: Option<String>,
}

impl Request {
    fn from_byte_array(buf -> &[u8]) -> Result <Self, String> {
        
    }
}

impl TryFrom for Request {
    
}