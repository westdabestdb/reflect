use crate::db::db::{Connection, IConnection};
use crate::models::user::{Login, Register, UpdateMe};
use crate::repositories::user_repository::{IUserRepository, UserRepository};
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpRequest, HttpResponse};

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

#[post("/login")]
async fn login(user: web::Json<Login>) -> HttpResponse {
  let _connection: Connection = Connection {};
  let _repository: UserRepository = UserRepository {
    connection: _connection.init(),
  };
  HttpResponse::Ok().json(_repository.login(user.into_inner()))
}

#[get("/me")]
async fn me(_req: HttpRequest) -> HttpResponse {
  let token = get_token(_req);
  let _connection: Connection = Connection {};
  let _repository: UserRepository = UserRepository {
    connection: _connection.init(),
  };
  match _repository.me(&token) {
    Ok(result) => HttpResponse::Ok().json(result.unwrap()),
    Err(err) => HttpResponse::Ok().json(err),
  }
}

#[post("/me/update")]
async fn me_update(_req: HttpRequest, data: web::Json<UpdateMe>) -> HttpResponse {
  let token = get_token(_req);
  let _connection: Connection = Connection {};
  let _repository: UserRepository = UserRepository {
    connection: _connection.init(),
  };
  match _repository.me_update(&token, &data.field, &data.value) {
    Ok(result) => HttpResponse::Ok().json(result),
    Err(err) => HttpResponse::Ok().json(err),
  }
}

fn get_token(_req: HttpRequest) -> String {
  let _auth = _req.headers().get("Authorization");
  let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
  let token = _split[1].trim();
  token.to_owned()
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(index);
  cfg.service(register);
  cfg.service(login);
  cfg.service(me);
  cfg.service(me_update);
}
