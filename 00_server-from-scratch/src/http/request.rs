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
        let string = String::from("asd");
        string.encrypt(); 
        buf.encrypt();
        unimplemented!()
    }
    
}

// trait Encrypt {
//     fn encrypt(&self) -> Self;
// }

// impl Encrypt for &[u8] {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }