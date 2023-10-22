
#![allow(dead_code)]

use std::env;
use server::Server;
use crate::website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let default_path = format!("{}/ppublic", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);
    let addr = String::from("127.0.0.1:8080");
    let port = &addr[10..];
    let server = Server::new(addr.to_string());
    server.run(WebsiteHandler::new(public_path));

    //let x = dbg!(port);
}
