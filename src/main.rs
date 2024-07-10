use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// Creating a Hello World Template
async fn greet(req: HttpRequest) -> impl Responder {
    // Name Makes a HTTP request, that gets the resource "name" and unwraps World
    // What is left is a String (Struct Responder) called Hello World
    let name = req.match_info().get("name").unwrap_or("World");

    format!("Hello {}", &name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("127.0.0.1:7878");
    // Handles Transport Layer TCP connections Security etc.
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
