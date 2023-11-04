use std::error::Error;
use std::fs;
use crate::http::{Request, Response, StatusCode};
use crate::http::method::Method;
use super::server::Handler;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub(crate) fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {

        let path = format!("{}/{}", self.public_path, file_path);
        println!("Reading file path {} ", path);

        match fs::canonicalize(path) {
            Ok(path) => {

                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(err) => {
                println!("ERROR trying to serve file: {}", err.to_string());
                None
            },
        }

        //fs::read_to_string(path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        println!("PROCESSING {},", request.path);
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => {

                        Response::new(StatusCode::Ok, Some(contents))
                    },
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}