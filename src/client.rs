use std::io::{Read, Write};
use std::net::TcpStream;

pub fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    loop {
        println!("Enter the message");
        let mut message = String::new();
        std::io::stdin().read_line(&mut message).unwrap();
        if message=="stop" {
          break;
        }

        let mut buffer = [0; 1024];

        stream.write_all(message.as_bytes()).expect("error");
        stream.read(&mut buffer).unwrap();

        let received = String::from_utf8_lossy(&buffer[..]);
        println!("Received: {}", received);
    }
}
