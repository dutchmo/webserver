use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode};
use std::net::{TcpListener, TcpStream};
use std::result::Result;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Server {
        Self { address }
    }

    pub fn run(self) -> std::io::Result<()>{
        println!("Server is listening.. {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_client(stream);
                }
                Err(e) => {
                    println!("Failed to read: {}", e);

                    /* failed */}

            }
        }

        return Ok(())
    }
}

fn handle_client(mut stream: TcpStream) {
    //let mut data: [i32; 4] = [1,2,3,4];
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read( &mut buffer);

    println!("Received a request: {}", String::from_utf8_lossy(&buffer));
    let response = match Request::try_from(&buffer[..]) {
        Ok(request) => {
            dbg!(request);
            let response = Response::new(StatusCode::Ok, Some("<h1> It works! </h1>".to_string()));
            response.send(&mut stream);
            // println!("Read request: {}", request.path)
        }
        Err(e) => {
            println!("Failed to read from connection: {}", e)
            Response::new(StatusCode::BadRequest, None).send(&mut stream);
        }
    }



    // todo!();
/*    'outer: loop {
        loop {
            continue 'outer;
        }
    }    */
}


