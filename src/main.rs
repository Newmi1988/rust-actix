use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Number {
    number: u32,
}

#[get("/prime")]
async fn check_prime(number : web::Query<Number>) -> String {
    format!("Possible prime number: {}", number.number)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(check_prime)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
