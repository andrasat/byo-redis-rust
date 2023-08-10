use byo_redis_rust::client::Client;
use byo_redis_rust::protocol_parser::message_builder;

#[async_std::main]
async fn main() {
    let mut client = Client::new("0.0.0.0", 8888).await;

    {
        let msg = "WHASUP";
        let body = message_builder(msg.to_string()).unwrap();
        client.send(&body).await;
    }

    {
        let msg = "Hello, world2!";
        let body = message_builder(msg.to_string()).unwrap();
        client.send(&body).await;
    }
}
