use async_std::{
    net::{TcpListener, TcpStream},
    stream::StreamExt,
};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
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

    pub async fn listen(&self, handle_connection: fn(stream: TcpStream)) {
        let mut incoming = self.listener.incoming();

        while let Some(stream) = incoming.next().await {
            match stream {
                Ok(stream) => handle_connection(stream),
                Err(e) => panic!("Failed to establish a connection: {}", e),
            };
        }
    }

    pub fn close(self) {
        drop(self.listener);
    }
}
