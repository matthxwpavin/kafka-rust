#![allow(unused_imports)]
use std::{io::Write, net::TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                _ = _stream.write_all(b"0\r\n7".as_slice());
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
