use tokio::task;
use actix_web::{post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use postgres::{Client, NoTls};

#[path = "../common/getpath.rs"]
mod getpath;
use getpath::{getpath, Conn};

#[path = "../common/getbody.rs"]
mod getbody;
use getbody::getbody;

#[derive(Serialize, Deserialize)]
struct Res {
    success: bool,
}

#[post("/connect")]
async fn connect(payload: web::Payload) -> Result<HttpResponse, Error> {
    let body: web::BytesMut = match getbody(payload).await {
        Ok(body) => body,
        Err(e) => return Err(e)
    };

    let path = getpath(serde_json::from_slice::<Conn>(&body).unwrap());

    let succ = task::spawn_blocking(move || validate(path.as_str())).await.unwrap();

    let res = Res {
        success: succ,
    };

    Ok(HttpResponse::Ok().json(res)) // <- send response
}

fn validate(path: &str) -> bool {
    match Client::connect(path, NoTls) {
        Ok(client) => {
            client.close().unwrap();
            return true;
        },
        Err(error) => {
            println!("Houve um problema na connexão com o banco de dados: {:?}", error);
            return false;
        }
    };
}