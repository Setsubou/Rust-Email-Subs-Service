use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

struct TestApp {
    address: String,
    db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    //Spawn a new instance of server and return its address along with the db pool
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let mut configuration = get_configuration().expect("Failed to read configuration");

    //Generate random database name for each test, the reason we are doing this is to make sure that each test
    //is being run on a fresh data set, once a random name has been set,
    //then we create a new DB by calling configure_test_database
    configuration.database.database_name = Uuid::new_v4().to_string();
    let db_pool = configure_test_database(&configuration.database).await;

    //Launch the server as background task
    let server = run(listener, db_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    //Return the final address and db pool of the server
    println!("Serving on {}", address);
    TestApp { address, db_pool }
}

pub async fn configure_test_database(config: &DatabaseSettings) -> PgPool {
    //Create database
    let mut connection =
        PgConnection::connect(&config.get_connection_string_without_db().expose_secret())
            .await
            .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    //Migrate database
    let connection_pool = PgPool::connect(&config.get_connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[actix_web::test]
async fn health_check_works() {
    // Setting up test
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    //Send request
    let response = client
        .get(&format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_web::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Setting up test
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    //Send Request
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    // Assert Response from server
    assert_eq!(200, response.status().as_u16());

    // Assert that Data has been successfully saved to database
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[actix_web::test]
async fn subscibe_returns_400_when_data_is_missing() {
    // Setting up test
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_data = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    // Send Request
    for (invalid_body, error_message) in test_data {
        let response = client
            .post(&format!("{}/subscriptions", &test_app.address))
            .header("Contet-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
