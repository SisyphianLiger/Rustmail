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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Creating the app
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscribe", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to Execute Request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_with_incorrect_data_returns_400() {
    //Creating app
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Make a Response
        let response = client
            .post(&format!("{}/subscribe", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to Execute Request.");

        // Assert to check that the status is 400 i.e. failed
        // But with a vector we can add a message to give specific info for
        // the failure
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customized information on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
