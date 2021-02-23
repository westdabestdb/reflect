use actix_web::{web, post, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct IndexResponse {
  message: String,
  success: bool,
}

#[derive(Deserialize)]
struct RegistrationData {
    fullname: String,
}

#[post("/register")]
async fn register(data: web::Json<RegistrationData>) -> Result<HttpResponse> {
  Ok(HttpResponse::Ok().json(IndexResponse {
    message: format!("hello {}!", data.fullname),
    success: true,
}))
}
