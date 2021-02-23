#[macro_use]
extern crate bson;
extern crate mongodb;

mod config;
mod db;
mod middlewares;
mod models;
mod repositories;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(web::scope("/api").configure(repositories::user_repository::init_routes))
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await
}
