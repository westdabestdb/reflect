use actix_web::{get, HttpResponse, Result};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct IndexResponse {
  message: String,
  success: bool,
}

#[get("/")]
pub async fn index() -> Result<HttpResponse> {
  Ok(HttpResponse::Ok().json(IndexResponse {
    message: "welcome to the api".to_string(),
    success: true,
  }))
}

#[get("/fields")]
pub async fn fields() -> Result<HttpResponse> {

  Ok(HttpResponse::Ok().json(IndexResponse {
    message: "welcome to the api".to_string(),
    success: true,
  }))

  // Ok(HttpResponse::Ok().json(UserInfo { full_name: val }))
}