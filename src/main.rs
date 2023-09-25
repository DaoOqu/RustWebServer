use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // We chose to handle any errors ungracefully with unwrap
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // result might be an error if the data isn't valid UTF-8 or if there was a problem reading from the stream.
    // We chose to handle any errors ungracefully with unwrap
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}