# rust-web-server
A basic HTTP web server made using Rust. The server concurrently handles TCP connections, and delivers HTTP responses to clients in response to valid HTTP requests.
All routes are currently defined and handled in `http.rs`.

To run the server, execute the following command in terminal:

`cargo run [host_ip] [port_number]`

e.g. `cargo run 127.0.0.1 9090`
