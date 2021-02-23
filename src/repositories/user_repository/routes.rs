
use crate::repositories::user_repository::{IUserRepository, UserRepository};
use crate::db::db::{Connection, IConnection};
use actix_web::{web, post, get, HttpResponse};
use actix_web::http::StatusCode;
use crate::models::user::{Register};

#[get("/")]
async fn index() -> HttpResponse {
  HttpResponse::new(StatusCode::OK)
    // let _connection: Connection = Connection {};
    // let _repository: UserRepository = UserRepository {
    //     connection: _connection.init(),
    // };
    // let proc = _repository.login(user.into_inner());

    // match proc {
    //     Ok(_) => HttpResponse::Ok().json(proc.unwrap()),
    //     Err(_) => HttpResponse::Ok()
    //         .status(StatusCode::from_u16(401).unwrap())
    //         .json(proc.unwrap_err()),
    // }
}

#[post("/register")]
async fn register(user: web::Json<Register>) -> HttpResponse {
    let _connection: Connection = Connection {};
    let _repository: UserRepository = UserRepository {
        connection: _connection.init(),
    };
    HttpResponse::Ok().json(_repository.register(user.into_inner()))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(index);
  cfg.service(register);
}