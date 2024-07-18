use std::net::TcpListener;

use zero2prod::configuration::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run(TcpListener::bind("127.0.0.1:0").expect("Cannot Bing to Port Specified, try again"))?.await
}
