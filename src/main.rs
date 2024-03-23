use secrecy::ExposeSecret;
use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    configure_logger();

    let configuration = get_configuration().expect("Failed to read configuration");

    let connection = PgPool::connect(
        configuration
            .database
            .get_connection_string()
            .expose_secret(),
    )
    .await
    .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    tracing::info!("Serving on http://{}", listener.local_addr().unwrap());

    run(listener, connection)?.await
}

fn configure_logger() {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
}
