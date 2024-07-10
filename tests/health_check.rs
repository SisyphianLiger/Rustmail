use reqwest::Client;
use std::net::TcpListener;

/*
 * tokio::test is testing equivalent of tokio::main
 * Spares you from #[test]
 *
 * You can inspect what code gets generatred using
 * cargo expand --test health_check (health_check == name of test file)
 *
 * */

fn spawn_app() -> String {
    // We see if we can bind a random port (the 0) from our OS
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to ramdom port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = newsletter::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arange
    let address = spawn_app();
    // We need to bring in request
    // to perform HTTP reqwests against our application
    let client = reqwest::Client::new();

    // Action
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert is good
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Action
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert is good
    assert_eq!(200, response.status().as_u16());
}
#[tokio::test]
async fn subscribe_returns_a_400_for_valid_form_data() {
    // Arrange Address and client
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    // Actions
    for (invalid_body, error_message) in test_cases {
        // Actions
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assertion
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
