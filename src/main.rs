use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::env;
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let hostname: &String= &args[1];
    let port: &String = &args[2];

    let listener = TcpListener::bind(format!("{}:{}", hostname, port)).unwrap();
    println!("Server is listening on {}:{}", hostname, port);
    
    for stream in listener.incoming() {
        handle_connection(stream?);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let handle = thread::spawn(move || {
        let mut buffer = [0; 10];
        stream.read(&mut buffer[..]);
        println!("Buffer: {:?}", buffer);

        for i in 1..30 {
            println!("{}", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
}