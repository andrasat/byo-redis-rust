use byo_redis_rust::client::Client;

fn main() {
    let client = Client::new("0.0.0.0", 8888);
    client.send("Hello, world!");
}
