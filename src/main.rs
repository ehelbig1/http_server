use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use http_message::Message;

fn main() {
    // todo add error handling for when binding fails
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        // todo add error handling for stream errors
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    Message::from_request(&buffer);
}
