use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read config
    let configuration = get_configuration().expect("Failed to read configuration");
    // We have removed the hard-coded val it's from settings
    
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to Connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
