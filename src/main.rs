use std::net::TcpListener;
use std::io::Write;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:3131")
    .expect("Could not bind to this address");

    print!("Waiting for clients to connect...");

    // 'incoming' returns an iterator over the received connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(_) = stream.write(b"hello world") {
                    println!("Failed to write hello to stream");
                }
            }
            Err(e) => {
                println!("This stream's connection has failed");
            }
        }
    }
}
