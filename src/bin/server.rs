use byo_redis_rust::server::Server;

#[async_std::main]
async fn main() {
    let mut server = Server::new("0.0.0.0", 8888).await;
    server.listen().await;
    server.close();
}
