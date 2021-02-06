use actix_web::{get, Error, HttpResponse};
use serde_json::Value;

#[get("/ai")]
pub async fn recommend(test: String) -> Result<HttpResponse, Error> {
    let body:Value = serde_json::from_str(&test)?;
    println!("{}",body["test"]);
    Ok(HttpResponse::Ok().body(test))
}