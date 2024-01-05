use std::net::TcpListener;

#[actix_web::test]
async fn health_check_works() {
    //Spawn our server and get its address
    let address = spawn_app();
     //Create new Reqwest client to make HTTP requests to our app
    let client = reqwest::Client::new();

    //Send request
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_web::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    //Send Request
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
    .post(&format!("{}/subscriptions", address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to send request");

    //Assert
    assert_eq!(200, response.status().as_u16());
}

#[actix_web::test]
async fn subscibe_returns_400_when_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_data = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_data {
        let response = client
        .post(&format!("{}/subscriptions", address))
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


fn spawn_app() -> String {
    //Spawn a new instance of server and return its address
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");

    //Launch the server as background task
    let _ = tokio::spawn(server);

    //Return the final address of the server
    let address = format!("http://127.0.0.1:{}", port);
    println!("Serving on {}", address);
    address
}
