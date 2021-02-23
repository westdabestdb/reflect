use actix_web::{get, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct IndexResponse {
  message: String,
  success: bool,
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
  Ok(HttpResponse::Ok().json(IndexResponse {
    message: "welcome to the api".to_string(),
    success: true,
}))
}
