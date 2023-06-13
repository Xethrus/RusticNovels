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

fn respond(stream: &mut TcpStream, html_file_name: &str) -> Result<()> {
    let wanted_html = format!("../{}",html_file_name);
    let content = get_html(wanted_html.as_str())?;
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn check_request(stream: &mut TcpStream, request_buffer: [u8; 1024]) -> Result<()> {
    let expected_request_start = b"GET / HTTP/1.1\r\n";
    if request_buffer.starts_with(expected_request_start) {
        respond(stream, "index.html");
        Ok(())
    } else {
        respond(stream, "404.html");
        Ok(())
    }
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
    check_request(&mut stream, buffer)?;
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


