use std::io::{Read, Write};
use std::net::{self, TcpStream};
use std::str;
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:2222").unwrap();
    stream.write("Here's Client End.".as_bytes()).unwrap();

    let mut buffer = [0u8; 50];
    stream.read(&mut buffer).unwrap();

    println!(
        "Response from server: {:?}",
        str::from_utf8(&mut buffer).unwrap()
    );
}
