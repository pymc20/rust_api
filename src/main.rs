use actix_web::{App, HttpServer};
use rust_api::zeplin;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(zeplin::webhook)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}