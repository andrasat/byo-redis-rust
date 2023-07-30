use async_std::{
    io::{ReadExt, WriteExt},
    net::{TcpListener, TcpStream},
    stream::StreamExt,
};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

async fn handle_connection(mut stream: TcpStream) {
    println!("Incoming connection from: {}", stream.peer_addr().unwrap());
    let mut buf = [0; 50];
    stream.read(&mut buf).await.unwrap();

    let text = String::from_utf8_lossy(&buf);
    println!("Received: {}", text);

    stream.write_all(&buf).await.unwrap();
    stream.flush().await.unwrap();
}

impl Server {
    pub async fn new(host: &str, port: u16) -> Self {
        let addr = format!("{}:{}", host, port);
        println!("Listening on {}", addr.as_str());

        let listener = match TcpListener::bind(addr).await {
            Ok(listener) => listener,
            Err(e) => panic!("Failed to bind to address: {}", e),
        };

        return Server { listener };
    }

    pub async fn listen(&mut self) {
        let mut incoming = self.listener.incoming();

        while let Some(stream) = incoming.next().await {
            match stream {
                Ok(stream) => handle_connection(stream).await,
                Err(e) => panic!("Failed to establish a connection: {}", e),
            };
        }
    }

    pub fn close(self) {
        drop(self.listener)
    }
}
