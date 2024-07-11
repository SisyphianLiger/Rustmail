use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn run() -> Result<(), std::io::Error> {
    println!("Go to 127.0.0.1:7878");

    // HttpServer handles the Transport Level
    HttpServer::new(|| {
        // Established a new connection with a client
        App::new()
            // The routes of requests we see here
            // Can also handle middleware etc
            .route("/health_check", web::get().to(health_check))
    })
    // The port we will bind to:w
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
