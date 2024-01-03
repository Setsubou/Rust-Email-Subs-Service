use std::net::TcpListener;

#[actix_web::test]
async fn health_check_works() {
    //Spawn our server and get its address
    let address = spawn_app();
    
    //Create new Reqwest client to make HTTP requests to our app
    let client = reqwest::Client::new();

    //Send request
    let response = client.get(&format!("{}/health_check", address))
    .send()
    .await
    .expect("Failed to execute request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");

    //Launch the server as background task
    let _ = tokio::spawn(server);

    //Return the final address of the server
    format!("http://127.0.0.1:{}", port)
}