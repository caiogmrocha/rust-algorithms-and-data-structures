use std::collections::HashMap;

use mini_redis::{Connection, Frame};
use mini_redis::Command::{self, Get, Set};

#[tokio::main]
async fn main() {
    let server = tokio::net::TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _ip) = server.accept().await.unwrap();

        tokio::spawn(async {
            run(socket).await;
        });
    }
}

async fn run(socket: tokio::net::TcpStream) {
    let mut connection = Connection::new(socket);

    let mut database: HashMap<String, Vec<u8>> = HashMap::new();

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let frame = match Command::from_frame(frame).unwrap() {
            Get(cmd) => {
                if let Some(value) = database.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            },
            Set(cmd) => {
                database.insert(cmd.key().to_string(), cmd.value().to_vec());

                Frame::Simple("OK".to_string())
            },
            cmd => panic!("UNIMPLEMENTED CMD: {:?}", cmd),
        };

        println!("GOT {frame:?}");

        connection.write_frame(&frame).await.unwrap();
    }
}