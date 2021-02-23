mod api;

use crate::api::index::index;
use actix_web::{App, HttpServer, web};


// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("welcome to the api v0.0.1")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(index)
        )
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await
}