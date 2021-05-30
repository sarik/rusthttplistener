use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
//use super::http::request::Request;
//use super::http::request::Request;   //super here is main which is also crate
use crate::http::request::Request;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(&self) {
        println!("Starting server at {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            println!("next iteration");
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    //println!("new client: {:?}", addr);
                    let mut mybuff = [0; 1024];
                    //stream.read(&stream, &mut mybuff);
                    match stream.read(&mut mybuff) {
                        Ok(_sizeofdata) => {
                            //println!("Received {} bytes of data", sizeofdata);
                            println!("the data is {}", String::from_utf8_lossy(&mybuff));

                            //let a = Request::try_from(&mybuff[..]);
                            match Request::try_from(&mybuff[..]) {
                                Ok(req) => {
                                    dbg!(req);
                                    //mybuff[1]=22;
                                    //mubuff has been borrowed to create req.Request struct members borrow mybuff.You cant modify the buffer while you are still using the req
                                    // let a= req;
                                }
                                Err(e) => println!("failed {}", e),
                            }
                        }
                        Err(e) => println!("Some error while reading the bytes {}", e),
                    }
                }
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    }
}
