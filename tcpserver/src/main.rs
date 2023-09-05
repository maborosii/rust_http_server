use std::{
    io::{Read, Write},
    net::{self, TcpListener},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:2222").unwrap();
    println!("listening port 2222...");

    // once
    // let _result = listener.accept().unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("connection established");

        let mut buffer = [0u8; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
