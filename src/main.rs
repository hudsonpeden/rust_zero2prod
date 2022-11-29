use std::net::TcpListener;

use rust_zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()>
{
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("failed to bind to port");
    run(listener)?.await
}
