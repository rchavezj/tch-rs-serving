use std::error::Error;
use std::convert::TryFrom;
use super::method::Method;
use std::fmt::{ 
    Debug,
    Display,
    Formatter, 
    Result as FmtResult
};


pub struct Request {
    path: String,
    method: Method,
    query_string: Option<String>,
}


impl Request {
    fn from_byte_array(buf -> &[u8]) -> Result <Self, String> { }
}


impl TryFrom<&[u8]> for Request {
    
    type Error = ParseError;

    // Get /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error>{
        let string = String::from("asd");
        string.encrypt(); 
        buf.encrypt();
        unimplemented!()
    }
}


pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}


impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid InvalidRequest",
            Self::InvalidEncoding => "Invalid InvalidEncoding",
            Self::InvalidProtocol => "Invalid InvalidProtocol",
            Self::InvalidMethod => "Invalid InvalidMethod"
        }
    }
}


impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message());
    }
}


impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message());
    }
}


impl Error for ParseError {

}
