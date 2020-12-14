use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self { Self { addr } }

    pub fn run (self) { 
        println!("Listening on {}", self.addr); 
        let listener = TcpListener::bind(&self.addr).unwrap(); 

        loop {
            match listener.accept() {
                Ok() => {
                    let a = 5;
                    println!("OK");
                },
                Err() => println!("Err")
            }
            let res = listener.accept();

            if res.is_err(){
                continue;
            }

            let (stream, addr) = res.unwrap();
        }
    }
}