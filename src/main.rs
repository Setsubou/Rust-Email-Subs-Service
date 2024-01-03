use std::net::TcpListener;

use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    println!("Serving on http://{}", listener.local_addr().unwrap());

    run(listener)?.await
}