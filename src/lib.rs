use std::net::TcpListener;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

#[get("/")]
async fn greet() -> impl Responder {
    return HttpResponse::Ok().body("Hello World! I'm alive and well");
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    return HttpResponse::Ok();
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
        .service(greet)
        .service(health_check)
    })
    .listen(listener)?
    .run();
    return Ok(server);
}
