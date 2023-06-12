use std::env;
use std::fs::{self, read_to_string};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

mod http;
use http::http::{HttpRequest, HttpResponse, HttpResponseHeader};

fn main() -> std::io::Result<()> {
    // env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();

    let hostname: &String = &args[1];
    let port: &String = &args[2];

    let listener = TcpListener::bind(format!("{}:{}", hostname, port)).unwrap();
    println!("Server is listening on {}:{}", hostname, port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!(
                    "Client connected from {}",
                    stream.peer_addr().unwrap().to_string()
                );
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    thread::spawn(move || {
        let mut buffer = [0; 2048];
        stream.read(&mut buffer[..]).unwrap();

        let request_string: &str = str::from_utf8(&buffer).unwrap();

        let request: HttpRequest = HttpRequest::from_string(request_string);

        let mut response: Vec<u8> = vec![];

        match request.get_method() {
            "GET" => match request.get_path() {
                "/" => {
                    let contents = read_to_string("html/index.html").expect("Unable to read file");
                    let headers: Vec<HttpResponseHeader> =
                        vec![HttpResponseHeader::new("Content-Type", "text/html")];
                    let response_struct: HttpResponse =
                        HttpResponse::new("200", "OK", headers, &contents);

                    response = response_struct.to_string().into_bytes();
                }
                "/register" => {
                    let contents =
                        read_to_string("html/register.html").expect("Unable to read file");
                    let headers: Vec<HttpResponseHeader> =
                        vec![HttpResponseHeader::new("Content-Type", "text/html")];
                    let response_struct: HttpResponse =
                        HttpResponse::new("200", "OK", headers, &contents);

                    response = response_struct.to_string().into_bytes();
                }
                &_ => {}
            },
            "POST" => match request.get_path() {
                "/create/user" => {
                    response = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\r\n\tOperation: createUser\r\n\tStatus: success\r\n}\r\n".to_vec();
                }
                &_ => {}
            },
            &_ => {}
        }

        println!("Received request: \n{}", request.to_string());
        stream.write_all(&response).unwrap();
        stream.flush().unwrap();
    });
}
