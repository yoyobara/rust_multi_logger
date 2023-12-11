use std::io::Write;

// struct server struct with generic output method
struct Server<W: Write> {
    output: W
}

impl<W: Write> Server<W> {

    /*
     * create a new Server with any output stream
     */
    pub fn new(output_stream: W) -> Server<W> {
        Server { output: output_stream }
    }
}

fn main() {
    let mut server = Server::new(std::fs::File::create("wow.txt",).unwrap());
}
