use actix_web::{post, web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[post("/subscriptions")]
async fn subscriptions(form: web::Form<FormData>) -> impl Responder {
    //Return 200 for now
    HttpResponse::Ok()
}