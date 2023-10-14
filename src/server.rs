use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::result::Result;

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
                    handle_client(stream?);
                }
                Err(e) => { /* failed */}

            }
        }

        return Ok(())
    }
}

fn handle_client(mut stream: TcpStream) {
    //let mut data: [i32; 4] = [1,2,3,4];
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read( &mut buffer);
    todo!();

/*    'outer: loop {
        loop {
            continue 'outer;
        }
    }    */
}


