use mini_redis::{client, server};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use rand::Rng;
use rand;
use std::time::SystemTime;


fn random_key() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut key: Vec<u8> = vec![];

    for _ in 0..15 {
        key.push(rng.gen_range(97..120));
    }

    key
}


async fn start_server() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move { server::run(listener, tokio::signal::ctrl_c()).await });

    addr
}


#[tokio::main]
async fn main() {
    let addr = start_server().await;

    let mut client = client::connect(addr).await.unwrap();

    let start = SystemTime::now();

    for _ in 0..100000 {

        let key = String::from_utf8(random_key()).unwrap();
        let value = random_key();

        client.set(&key, value.clone().into()).await.unwrap();

        assert_eq!(client.get(&key).await.unwrap().unwrap(), &value[..])
    }

    let end = SystemTime::now();

    let x = end.duration_since(start).unwrap();
    println!("{}", x.as_millis());
}