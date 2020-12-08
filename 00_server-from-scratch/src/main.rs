struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self { Self { addr } }

    fn run (self) { println!("Listening on {}", self.addr); }
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}


// &self --> a string slice is an immutable reference to a part of a string. 
// 
fn main() {

    // let get = Method::GET("abcd".to_string());
    // let put = Method::PUT;
    // let post = Method::POST;
    // let delete = Method::DELETE(100);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();    
}


