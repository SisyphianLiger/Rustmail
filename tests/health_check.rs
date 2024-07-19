use std::net::TcpListener;
use sqlx::{Connection, PgConnection, PgPool};
use zero2prod::configuration::{self, get_configuration};
use zero2prod::startup::run;




#[tokio::test]
async fn health_check_works() {
    // Now spawn_app outputs the address
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        // and we can use that for the get requst test
        .get(&format!("{}/health_check", &address.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    // Setting up a gateway with the 0 to put an OS port available
    let http = "http://127.0.0.1:";

    // Call TCP listener to bind with fail attached
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to address");

    // ripping port to display in terminal
    let port = listener.local_addr().unwrap().port();

    // Making sure we get a configuration for run for the Pg db
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres.");

    // Running server and spawning as Future
    let server = run(listener, connection_pool.clone())
        .expect("Failed to bind Address");

    let _ = tokio::spawn(server);

    // returning String for the caller
    let address = format!("{}{}", http, port);
    TestApp {
        address,
        db_pool: connection_pool,
    }
        
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Creating the app
    let app_address = spawn_app().await;
    // The 'Connection' trait MUST be in scope for us to invoke 
    // PGConnection::connect - it is not an inherent method of the struct
    
    // Now we make a client request
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to Execute Request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email,name FROM subscriptions",).fetch_one(&app_address.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");

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
        println!("The Invalid Body is {}", &invalid_body);
        let response = client
            .post(&format!("{}/subscriptions", &app_address.address))
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
