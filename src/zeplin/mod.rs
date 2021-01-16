mod note;
mod screen;
use actix_web::{post, Error, HttpResponse};
use serde_json::Value;

#[post("/zeplin")]
pub async fn webhook(test: String) -> Result<HttpResponse, Error> {
    let body:Value = serde_json::from_str(&test)?;
    println!("{}",body["test"]);
    Ok(HttpResponse::Ok().body(test))
}