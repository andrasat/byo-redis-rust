use std::{
    io::{Read, Write},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(host: &str, port: u16) -> Self {
        let addr = format!("{}:{}", host, port);
        println!("Connecting to {}", addr.as_str());

        let stream = match TcpStream::connect(addr) {
            Ok(stream) => stream,
            Err(e) => panic!("Failed to connect to address: {}", e),
        };

        return Client { stream };
    }

    pub fn send(mut self, message: &str) {
        self.stream.write(message.as_bytes()).unwrap();

        let mut data = [0 as u8; 1024];
        self.stream.read(&mut data).unwrap();

        let text = String::from_utf8_lossy(&data);
        println!("Received: {}", text);
        self.stream.flush().unwrap();

        drop(self.stream)
    }
}
