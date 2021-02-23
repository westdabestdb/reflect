mod api;
mod user_enum;

use crate::api::development::register;
use crate::api::public::fields;
use crate::api::public::index;
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
                    .service(fields) // list of fields
                    .service(
                        web::scope("/dev")
                            .service(index)
                            .service(register)
                    )
            )
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await
}