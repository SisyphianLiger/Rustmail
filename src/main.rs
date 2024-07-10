use::zero2prod::run;
use std::net::TcpListener;
/*
        HttpServer --> Handles incoming requests, and is the transport layer
        Application --> Routing/Middleware/requests exists
        Endpoint(.route) --> creates the webpath end 

 */



// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");

    run(listener)?.await
}

