

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self { Self { addr } }

    fn run (self) { println!("Listening on {}", self.addr); }
}


struct Request {
    path: String,
    query_string: String,
    method: String,
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

// &self --> a string slice is an immutable reference to a part of a string. 
// 
fn main() {

    let get = Method::GET;
    let put = Method::PUT;
    let post = Method::POST;
    let delete = Method::DELETE;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();    
}


