mod api;

use crate::api::public::register::register;
use crate::api::public::index::index;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                web::scope("/api")
                    .service(index) // public api index
                    .service(register) // user register
            )
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await
}