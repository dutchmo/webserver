use std::net::{TcpListener, TcpStream};
use std::result::Result;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Server {
        Self { address }
    }

    pub fn run(self) -> (i32, i32) {
        println!("Server is listening.. {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();
        for stream in listener.incoming() {
            handle_client(stream?);
        }

        let tup = (3,4);
        return tup;
        // Result::OK(());
    }
}

fn handle_client(p0: TcpStream) {
    todo!();

/*    'outer: loop {
        loop {
            continue 'outer;
        }
    }    */
}


