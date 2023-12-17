mod common;
use std::{io::{Write, stdout}, net::{TcpListener, TcpStream}, error::Error, thread, sync::{Arc, Mutex}};
use common::sock::Protocol;

struct Client {
    stream: TcpStream,
    name: String,
}

/*
 * a handler for a client using TcpStream
 */
fn handle_client<T: Write + Send>(client: Client, output_stream: Arc<Mutex<T>>) {
    println!("{}", client.name);
    log(output_stream, &format!("A client named \"{}\" has joined!", client.name));
}

fn log<T:Write + Send>(output_stream: Arc<Mutex<T>>, data: &str) {
    let mut output_acquired = output_stream.lock().unwrap();
    output_acquired.write_all(data.as_bytes()).unwrap();
    output_acquired.write_all(b"\n").unwrap();
    output_acquired.flush().unwrap();
}

/*
 * start the logging server on `port`
 * outputing the log to `output`
 */
fn start_server<T: Write + Send + 'static>(port: u16, output_stream: T) -> Result<(), Box<dyn Error>>{
    // listen to incoming connections
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    let out = Arc::new(Mutex::new(output_stream));

    for new_connection in listener.incoming(){
        let mut client_stream = new_connection.unwrap();
        let out_clone = out.clone();

        let (0x6A, data) = client_stream.recieve_message() else {panic!()};
        let cl = Client{ name: String::from_utf8(data)?, stream: client_stream};
        
        thread::spawn(move ||{
            handle_client(cl, out_clone);
        });
    }

    Ok(())
}

fn main() {
    let out = stdout();
    start_server(5000, out).unwrap();
}
