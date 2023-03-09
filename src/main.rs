use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    println!("new client connected!");
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:3131")
    .expect("Could not bind to this address");

    print!("Waiting for clients to connect...");

    // 'incoming' returns an iterator over the received connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move ||{
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("This stream's connection has failed");
            }
        }
    }
}
