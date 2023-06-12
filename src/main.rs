use std::env;
mod http;

fn main() -> std::io::Result<()> {
    // env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();

    let hostname: &String = &args[1];
    let port: &String = &args[2];

    http::http::listen(hostname, port);

    Ok(())
}