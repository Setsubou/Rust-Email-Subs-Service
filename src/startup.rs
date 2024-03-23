use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes::*;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    // Wrap PgConnection in Arc pointer since HtppServer App need a cloneable reference
    let db_pool = web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(greet)
            .service(health_check)
            .service(subscriptions)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
