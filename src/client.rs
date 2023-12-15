use std::{net::TcpStream, io::Write};

fn main() {
    let mut s = TcpStream::connect("127.0.0.1:5000").unwrap();
    s.write_all("hello".as_bytes()).unwrap();
}
