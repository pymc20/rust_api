mod makes;
use actix_web::{post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct Test {
    test: String
}

#[post("/zeplin")]
pub async fn webhook(test: web::Json<Test>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(serde_json::to_string(&test.into_inner()).unwrap()))
}