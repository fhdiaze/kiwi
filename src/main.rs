use crate::modules::race;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

mod domain;
mod modules;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

    let r = race::controller::handle_get();
    println!("{:?}", r);
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, content_path) =
        if buffer.starts_with(get) {
            let races = race::controller::handle_find();
            println!("{:?}", races);
            ("200 OK", "index.html")
        } else if buffer.starts_with(sleep)  {
            thread::sleep(Duration::from_secs(5));
            ("200 OK", "index.html")
        } else {
            ("404 NOT FOUND", "404.html")
        };

    let content = fs::read_to_string(content_path).unwrap();
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
