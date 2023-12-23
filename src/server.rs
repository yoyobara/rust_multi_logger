mod common;
use std::{io::{Write, stdout}, net::{TcpListener, TcpStream}, error::Error, thread, sync::{Arc, Mutex}};
use common::sock::{Protocol, MSG_LOG, MSG_JOIN, MSG_JOIN_OK, MSG_LEAVE};

struct Client {
    stream: TcpStream,
    name: String,
}

/*
 * a handler for a client using TcpStream
 */
fn handle_client<T: Write + Send>(client: Client, output_stream: Arc<Mutex<T>>) -> Result<(), Box<dyn Error>>{
    let Client{name, stream: mut client_stream} = client;

    client_stream.send_message(MSG_JOIN_OK, &[]);

    log(output_stream.clone(), &format!("[A client named \"{}\" has joined!]", name));

    loop {
        match client_stream.recieve_message() {
            (MSG_LOG, data) => {
                log(output_stream.clone(), &String::from_utf8(data)?)
            }

            (MSG_LEAVE, _) => {
                log(output_stream.clone(), &format!("[{} just left the server!]", name));
                return Ok(())
            }

            (k, _) => {
                return Err(format!("unknown type of msg: {}", k).into())
            }
        }
    }
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

    log(out.clone(), "logging server started!");

    for new_connection in listener.incoming(){
        let mut client_stream = new_connection.unwrap();
        let out_clone = out.clone();

        let (MSG_JOIN, data) = client_stream.recieve_message() 
        else {
            return Err("client hasn't joined".into())
        };

        let cl = Client{ name: String::from_utf8(data)?, stream: client_stream};
        
        thread::spawn(move ||{
            handle_client(cl, out_clone).unwrap();
        });
    }

    Ok(())
}

fn main() {
    let out = stdout();
    start_server(5001, out).unwrap();
}
