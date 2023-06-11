use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use anyhow::Result;
use anyhow::Error;


fn bind_listener(uri: &str) -> Result<TcpListener, Error> {
   let listener = TcpListener::bind(uri)?; 
   println!("succesfully bound");
   Ok(listener)
}

fn handle_stream(listener: TcpListener) -> Result<Vec<TcpStream>, Error> {

    let mut stream_vec: Vec<TcpStream> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream?;
        println!("connection established");
        stream_vec.push(stream);
    }    
    Ok(stream_vec)
}


fn main() {
    let listener = match bind_listener("127.0.0.1:8000") {
        Ok(listener) => listener,
        Err(e) => {
            panic!("was unable to bind listener: {}", e);
        }
    };
    let connections = match handle_stream(listener) {
        Ok(listener) => listener,
        Err(e) => {
            panic!("was unable to retrieve conenctions: {}", e);
        }
    }; 
}
