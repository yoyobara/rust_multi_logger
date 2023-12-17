mod common;
use std::{net::TcpStream, io::Write};
use common::sock::Protocol;

fn main() {
    let mut s = TcpStream::connect("127.0.0.1:5000").unwrap();
    s.send_message(0x6A, b"yoti");
}
