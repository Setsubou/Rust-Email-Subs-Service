use secrecy::ExposeSecret;
use std::net::TcpListener;

use sqlx::{postgres::PgPoolOptions, PgPool};
use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

fn configure_logger() {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    configure_logger();

    let configuration = get_configuration().expect("Failed to read configuration");

    let connection = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(10))
        .connect_lazy(&configuration.database.get_connection_string().expose_secret())
        .expect("Unable to connect to Postgres.");

    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    tracing::info!("Serving on http://{}", listener.local_addr().unwrap());

    run(listener, connection)?.await
}