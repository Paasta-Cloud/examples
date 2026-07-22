use std::{env, io::{Read, Write}, net::TcpListener};

fn main() {
    let port = env::var("PORT").unwrap_or("3000".into());
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut request = [0; 1024];
        let _ = stream.read(&mut request);
        let body = "Paasta Rust example";
        let _ = write!(stream, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body);
    }
}
