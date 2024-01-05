use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello World! I'm alive and well")
}

#[post("/subscriptions")]
async fn subscriptions() -> impl Responder {
    //Return 200 for now
    HttpResponse::Ok()
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new()
        .service(greet)
        .service(health_check)
        .service(subscriptions))
        .listen(listener)?
        .run();
    Ok(server)
}
