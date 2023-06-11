use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use anyhow::Result;
use anyhow::Error;
//use std::thread;
use std::fs;


fn bind_listener(uri: &str) -> Result<TcpListener, Error> {
   let listener = TcpListener::bind(uri)?; 
   println!("succesfully bound to: {}", uri);
   Ok(listener)
}

fn collect_stream(listener: TcpListener) -> Result<()> {
    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream)?;
    }
    Ok(())
}

fn respond(stream: &mut TcpStream) -> Result<()> {
    let content = get_html("../index.html")?;
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn get_html(file_name: &str) -> Result<String> {
    let contents = fs::read_to_string(file_name)?;
    Ok(contents)
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    println!("connection being handled");
    let mut buffer = [0;1024]; 
    stream.read(&mut buffer)?;
    println!(
       "Request: {}",
        String::from_utf8_lossy(&buffer[..])
    );
    respond(&mut stream)?;
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
}


