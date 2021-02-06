use actix_web::{App, HttpServer, middleware, HttpRequest, HttpResponse, Error, web};
use rust_api::{ai};
use openssl::ssl::{SslAcceptor, SslMethod, SslFiletype};

async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
            .service(ai::recommend)
    })
    .bind_openssl("0.0.0.0:8000", builder)?
    .run()
    .await
}