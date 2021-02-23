use actix_web::{web, get, post, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct IndexResponse {
  message: String,
  success: bool,
}

#[get("/")]
pub async fn index() -> Result<HttpResponse> {
  Ok(HttpResponse::Ok().json(IndexResponse {
    message: "welcome to the developer api".to_string(),
    success: true,
  }))
}


#[derive(Deserialize)]
pub struct RegisterData {
    fullname: String,
    username: String,
    password: String,
}

#[post("/register")]
pub async fn register(data: web::Json<RegisterData>) -> Result<HttpResponse> {
  Ok(HttpResponse::Ok().json(IndexResponse {
    message: format!("hello {}", data.fullname),
    success: true,
  }))
}