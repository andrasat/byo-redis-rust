use async_std::{io::WriteExt, net::TcpStream};

use crate::{protocol_parser::message_parser, r#const::MAX_MSG};

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub async fn new(host: &str, port: u16) -> Self {
        let addr = format!("{}:{}", host, port);
        println!("Connecting to {}", addr.as_str());

        let stream = match TcpStream::connect(addr).await {
            Ok(stream) => stream,
            Err(e) => panic!("Failed to connect to address: {}", e),
        };

        return Client { stream };
    }

    pub async fn send(&mut self, message: &str) {
        let mut stream_clone = self.stream.clone();

        match stream_clone.write(message.as_bytes()).await {
            Ok(n) => println!("Wrote {} bytes", n),
            Err(e) => panic!("Failed to write to stream: {}", e),
        };
        stream_clone.flush().await.unwrap();

        let buf = [0; MAX_MSG];
        let (len, data) = message_parser(&stream_clone, buf).await;

        println!("Received {} bytes", len);
        println!("Response from server: {}", data);
    }
}
