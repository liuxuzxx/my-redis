use std::collections::HashMap;

use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let addr = String::from("127.0.0.1:7379");
    println!("Start mini redis server on {addr}");
    let listener = TcpListener::bind(addr).await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    let mut db: HashMap<String, String> = HashMap::new();

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Get :{:?}", frame);
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
