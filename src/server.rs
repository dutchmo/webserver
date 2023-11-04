use std::io::{Read, Write};
use crate::http::{ParseError, Request, Response, StatusCode};
use std::net::{TcpListener, TcpStream};
use std::result::Result;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::process;
use std::process::ExitCode;


pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Server {
        Self { address }
    }

    pub fn run(self, mut handler: impl Handler) -> ExitCode {
        println!("Server is listening.. {}", self.address);

        let listener = match TcpListener::bind(&self.address) {
            Ok(listener) => listener,
            Err(e) => {
                println!("Failed to start server: {}", e);
                // process::exit(-1);
                return ExitCode::from(1);
            }
        };

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_client(stream, &mut handler);
                }
                Err(e) => {
                    println!("Failed to read: {}", e);
                    /* failed */
                }
            }
        }
        return ExitCode::from(0);
    }
}

fn handle_client(mut stream: TcpStream, mut handler: &mut impl Handler) {
    //let mut data: [i32; 4] = [1,2,3,4];
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer);

    println!("Received a request: {}", String::from_utf8_lossy(&buffer));
    let response = match Request::try_from(&buffer[..]) {
        Ok(request) => handler.handle_request(&request),
        Err(e) => handler.handle_bad_request(&e),
    };


    if let Err(e) = response.send(&mut stream) {
        println!("Failed to send response: {}", e);

        // todo!();
        /*    'outer: loop {
                loop {
                    continue 'outer;
                }
            }    */
    }
}


