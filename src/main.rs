use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //Initialize tracing at info level or above
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    //Read configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    //Connect to Postgres
    let connection = PgPool::connect(&configuration.database.get_connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    // Get the server address and start it
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    log::info!("Serving on http://{}", listener.local_addr().unwrap());

    run(listener, connection)?.await
}
