use std::net::TcpListener;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8080").expect("Failed to bind port");
    run(listener)?.await
}
