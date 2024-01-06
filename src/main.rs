use std::net::TcpListener;

use zero2prod::{startup::run, configuration::get_configuration};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //Read configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    //Create server
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    println!("Serving on http://{}", listener.local_addr().unwrap());

    run(listener)?.await
}
