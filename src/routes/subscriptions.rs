use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[post("/subscriptions")]
async fn subscriptions(form: web::Form<FormData>, connection: web::Data<PgPool>) -> impl Responder {
    //Generate a random request ID for each request
    let request_id = Uuid::new_v4();
    tracing::info!(
        "request id {} - Received request to save new subscriber, {} - {}",
        request_id,
        form.email,
        form.name
    );

    // Execute Query and match its results
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok()
        }
        Err(error) => {
            tracing::error!(
                "request id {} - Failed to execute query: {:?}",
                request_id,
                error
            );
            HttpResponse::InternalServerError()
        }
    }
}
