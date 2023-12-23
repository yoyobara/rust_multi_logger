mod common;
use std::{net::{TcpStream, ToSocketAddrs}, io::Write, fmt::Display};
use common::sock::{Protocol, MSG_LOG, MSG_LEAVE, MSG_JOIN, MSG_JOIN_OK};

#[derive(Debug)]
struct LoggingClient {
    server: TcpStream,
    name: String,
}

impl LoggingClient {

    fn connect<A: ToSocketAddrs>(address: A, name: String) -> Result<LoggingClient, String> {
        TcpStream::connect(address).map_err(|_| "internet bad").and_then(|mut conn| {

            conn.send_message(MSG_JOIN, name.as_bytes());
            if let (MSG_JOIN_OK, _) = conn.recieve_message() {
                Ok(LoggingClient { server: conn, name })
            } else {
                Err("didnt get join_ok")
            }

        }).map_err(|_| "shit".to_owned())
    }

    fn log(&mut self, content: &str) {
        self.server.send_message(MSG_LOG, content.as_bytes());
    }
}

impl Drop for LoggingClient {
    fn drop(&mut self) {
        self.server.send_message(MSG_LEAVE, &[]);
        println!("drop");
    }
}

fn main() {
    let mut logger = LoggingClient::connect("127.0.0.1:5001", "yotibayoti".to_owned()).unwrap();

    logger.log("wow are you real?");
}
