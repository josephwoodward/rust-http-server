use std::{io::Read, net::TcpListener};

use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let _listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match _listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            if let Err(e) = Request::try_from(&buf[..]) {
                                println!("Failed to parse request {}", e)
                            }
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                        }
                        Err(e) => println!("Failed to read from connection {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection {}", e),
            }
        }
    }
}
