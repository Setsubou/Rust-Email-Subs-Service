use actix_web::{Responder, HttpResponse, get};

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello World! I'm alive and well")
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

