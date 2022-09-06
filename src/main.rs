use std::env;

use actix_web::{HttpServer, web, App};

mod routes;

use routes::listtables::listtables;
use routes::connect::connect;
use routes::getgeojson::getgeojson;
use routes::test::test;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    HttpServer::new(|| {
        App::new()
            .service(listtables)
            .service(connect)
            .service(getgeojson)
            .route("/test", web::get().to(test))
    })
    .bind(args.get(1).expect("Argumento faltando!"))
    .expect("Porta ocupada!")
    .run()
    .await
}