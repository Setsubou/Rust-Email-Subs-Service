use actix_web::{dev::Server, App, HttpServer};
use std::net::TcpListener;

use crate::routes::*;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new()
        .service(greet)
        .service(health_check)
        .service(subscriptions))
        .listen(listener)?
        .run();
    Ok(server)
}