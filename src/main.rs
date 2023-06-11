use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use anyhow::Result;
use anyhow::Error;
use std::thread;


fn bind_listener(uri: &str) -> Result<TcpListener, Error> {
   let listener = TcpListener::bind(uri)?; 
   println!("succesfully bound to: {}", uri);
   Ok(listener)
}

fn collect_stream(listener: TcpListener) -> Result<Vec<TcpStream>, Error> {

    let mut stream_vec: Vec<TcpStream> = Vec::new();
    println!("listening...");
    for stream in listener.incoming() {
        let stream = stream?;
        println!("connection established");
        stream_vec.push(stream);
    }    
    Ok(stream_vec)
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    println!("connection being handled");
    let mut buffer = [0;1024]; 
    stream.read(&mut buffer)?;
    println!(
       "Request: {}",
        String::from_utf8_lossy(&buffer[..])
    );
    Ok(())
}

fn handle_connections(stream_vec: Vec<TcpStream>) -> Result<()> {
    println!("starting handling loop");
    for stream in stream_vec {
        handle_connection(stream)?;
    }
    Ok(())
}


fn main() {
    let listener = match bind_listener("127.0.0.1:7878") {
        Ok(listener) => listener,
        Err(e) => {
            panic!("was unable to bind listener: {}", e);
        }
    };
    let connections = match collect_stream(listener) {
        Ok(connections) => connections,
        Err(e) => {
            panic!("was unable to retrieve conenctions: {}", e);
        }
    }; 
    match handle_connections(connections) {
        Ok(()) => {
            println!("successfully handled connections");
        }
        Err(e) => {
            panic!("was unable to handle conenctions: {}", e);
        }
    }
//    let handle = thread::spawn(move || {
//        match collect_stream(listener) {
//            Ok(connections) => {
//                match handle_connections(connections) {
//                    Ok(()) => {
//                        println!("successfully handled connections");
//                    }
//                    Err(e) => {
//                        panic!("was unable to handle connections: {}", e);
//                    }
//                }
//            }
//            Err(e) => {
//                panic!("was unable to retrieve connections: {}", e);
//            }
//        }
//    });
//    handle.join().expect("failed to join thread");
}


