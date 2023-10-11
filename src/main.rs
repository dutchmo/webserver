fn main() {
    let addr = String::from("192.168.0.0:8080");
    let port = &addr[10..];
    let server = Server::new("addr".to_string());
    server.run();

    let x = dbg!(port);
}

struct Server {
    address: String,
}

impl Server {
    fn new(address: String) -> Server {
        Self { address }
    }

    fn run(self) {
        println!("Server is listening..")
    }
}
 struct Request {
     path: String,
     query_string: Option<String>,
     method: Method,
 }

enum Method {
    GET(String),
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}