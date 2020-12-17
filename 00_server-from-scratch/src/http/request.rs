use super::method::Method;
use std::convert::TryFrom;
        
pub struct Request {
    path: String,
    method: Method,
    query_string: Option<String>,
}

impl Request {
    fn from_byte_array(buf -> &[u8]) -> Result <Self, String> { }
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error>{
        unimplemented!()
    }
    
}