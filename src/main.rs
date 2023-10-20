
#![allow(dead_code)]

use server::Server;
use http::Request;

mod server;
mod http;

fn main() {
    let addr = String::from("127.0.0.1:8080");
    let port = &addr[10..];
    let server = Server::new(addr.to_string());
    server.run();

    //let x = dbg!(port);
}
