

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self { Self { addr } }

    fn run (&mut self) {
        
    }
}


// &self --> a string slice is an immutable reference to a part of a string. 
// 
fn main() {
    let foodieString = String::from("🍔🍟🍻🍻");
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..];
    let string_borrow: &str = &string;
    let string_literal = "1234";

    dbg!(&string);
    dbg!(&string_slice);
    dbg!(&string_borrow);
    dbg!(&string_literal);

    let server = Server::new("127.0.0.1:8080".to_string());
    // server.run();    
}


