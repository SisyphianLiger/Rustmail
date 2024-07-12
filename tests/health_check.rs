use std::net::TcpListener;

use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    // Now spawn_app outputs the address
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        // and we can use that for the get requst test
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    // Setting up a gateway with the 0 to put an OS port available
    let http = "http://127.0.0.1:";

    // Call TCP listener to bind with fail attached
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to address");

    // ripping port to display in terminal
    let port = listener.local_addr().unwrap().port();

    // Running server and spawning as Future
    let server = run(listener).expect("Failed to bind Address");
    let _ = tokio::spawn(server);

    // returning String for the caller
    format!("{}{}", http, port)
}
