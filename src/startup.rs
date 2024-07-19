use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::{PgConnection, PgPool};
use std::net::TcpListener;
use actix_web::middleware::Logger;

use crate::routes::health_check;
use crate::routes::subscribe;


pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Arc<T? is actually web::Data::new() under the hood, and allows for cloneability
    let connection = web::Data::new(db_pool);
    // Give posession of the connection here with the move keyword
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(health_check))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}


