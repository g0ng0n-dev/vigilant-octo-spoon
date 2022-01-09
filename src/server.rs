use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Server{
    address: String,
}

impl Server{
    pub fn new(address: String) -> Self{
        Self { 
            address 
        }
    }

    pub fn run(self){
        println!("Listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {

            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer= [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Receveid a request: {}", String::from_utf8_lossy(& buffer))
                            match Request::try_from(&buffer[..]) {
                                Ok(request)=> {},
                                Err(e) => println!("Error {}", e);
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e)
                    };
                },
                Err(e) => println!("Failes to establish a connect: {}", e)
            }
        }
    }
}