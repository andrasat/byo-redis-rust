use std::os::fd::AsRawFd;

use async_std::{
    io::WriteExt,
    net::{TcpListener, TcpStream},
    task::spawn,
};
use futures::StreamExt;
use nix::sys::socket::setsockopt;
use nix::sys::socket::sockopt::ReuseAddr;

use crate::{protocol_parser::message_parser, r#const::MAX_MSG};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

async fn handle_connection(mut stream: TcpStream) {
    let buf = [0; MAX_MSG];
    let (_, data) = message_parser(&stream, buf).await;

    let final_msg = format!("{}{}", data, ", from server!");
    let final_len = final_msg.len();

    let response = format!("{}\r\n{}", final_len, final_msg);
    match stream.write(response.as_bytes()).await {
        Ok(n) => println!("Wrote {} bytes", n),
        Err(e) => panic!("Failed to write to stream: {}", e),
    };

    match stream.flush().await {
        Ok(_) => println!("Flushed"),
        Err(e) => panic!("Failed to flush stream: {}", e),
    };
}

impl Server {
    pub async fn new(host: &str, port: u16) -> Self {
        let addr = format!("{}:{}", host, port);
        println!("Listening on {}", addr.as_str());

        let listener = match TcpListener::bind(addr).await {
            Ok(listener) => listener,
            Err(e) => panic!("Failed to bind to address: {}", e),
        };

        let fd = listener.as_raw_fd();
        setsockopt(fd, ReuseAddr, &true).unwrap();

        return Server { listener };
    }

    pub async fn listen(&mut self) {
        self.listener
            .incoming()
            .for_each_concurrent(None, |stream| async {
                match stream {
                    Ok(stream) => {
                        println!("Incoming connection from: {}", stream.peer_addr().unwrap());
                        spawn(handle_connection(stream))
                    }
                    Err(e) => panic!("Failed to establish a connection: {}", e),
                };
            })
            .await
    }

    pub fn close(self) {
        drop(self.listener)
    }
}
