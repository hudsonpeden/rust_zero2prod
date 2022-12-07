use std::net::TcpListener;
use rust_zero2prod::startup;

#[tokio::main]
async fn main() -> std::io::Result<()>
{
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("failed to bind to port");
    startup::run(listener)?.await
}
