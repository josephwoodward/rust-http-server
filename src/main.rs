fn main() {
    let server = Server::new("127.0.0.1:8080".to_owned());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        print!("Listening on {}", self.addr)
    }
}
