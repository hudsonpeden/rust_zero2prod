use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("failed to bind to port");
    startup::run(listener, connection_pool)?.await
}
