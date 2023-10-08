fn main() {

    let addr = String::from("192.168.0.0:8080");
    let port = &addr[10..];
    let server = Server::new(addr);

    dbg!(port)

}

struct Server {
    address: String
}

impl Server {
    fn new(address: String) -> Server {
        Self {address}
    }

}
