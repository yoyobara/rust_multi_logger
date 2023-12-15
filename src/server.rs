use std::{io::{Write, stdout}, net::{TcpListener, TcpStream}, error::Error, thread, sync::{Arc, Mutex}, fs::File, any::type_name};

/*
 * a handler for a client using TcpStream
 */
fn handle_client<T: Write + Send>(client: TcpStream, output_stream: Arc<Mutex<T>>) {
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
        let client_stream = new_connection.unwrap();
        let out_clone = out.clone();
        
        thread::spawn(move ||{
            handle_client(client_stream, out_clone);
        });
    }

    Ok(())
}

fn main() {
    let out = stdout();
    start_server(5000, out).unwrap();
}
