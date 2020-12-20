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


// impl Request {
//     fn from_byte_array(buf -> &[u8]) -> Result <Self, String> { }
// }


impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    
    // Get /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error>{
        let request = str::from_utf8(buf)?;
        
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol)
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) == path.find('?') {
            query_string = Some(&path[i + 1..]);    // +1 byte
            path = &path[..i];
        }
        
        unimplemented!()
    }
}


fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let mut iter = request.chars();

    // enumerate gives users to also' 
    // get current index when looping
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((
                &request[..i], 
                &request[i + 1..]
            ));
        }
    }
    None
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


impl From<Utf8Error> for ParseError {

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
