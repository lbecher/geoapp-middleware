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
struct Table {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Res {
    tables: Vec<Table>,
}

#[post("/listtables")]
async fn listtables(payload: web::Payload) -> Result<HttpResponse, Error> {
    let body: web::BytesMut = match getbody(payload).await {
        Ok(body) => body,
        Err(e) => return Err(e)
    };

    let path = getpath(serde_json::from_slice::<Conn>(&body).unwrap());

    let list = task::spawn_blocking(move ||
        pg_listtables(path.as_str())
    ).await.unwrap().unwrap();

    let mut res = Res {
        tables: Vec::new()
    };

    for string in list {
        res.tables.push(Table {name: string});
    }

    Ok(HttpResponse::Ok().json(res)) // <- send response
}

fn pg_listtables(path: &str) -> Result<Vec<String>, String> {
    let mut client = match Client::connect(path, NoTls) {
        Err(_e) => return Err("Erro ao conectar ao banco de dados!".to_string()),
        Ok(client) => client,
    };

    let mut list: Vec<String> = Vec::new();

    let query = "SELECT f_table_name FROM geometry_columns WHERE (f_table_schema = 'public' AND f_geometry_column = 'geom' AND coord_dimension = 2 AND (type = 'POINT' OR type = 'MULTIPOLYGON' OR type = 'MULTILINESTRING'))".to_string();

    for row in client.query(query.as_str(), &[]).unwrap() {
        let table: &str = row.get(0);
        list.push(table.to_string());
    }

    match client.close()  {
        Err(_e) => return Err("Não foi possível fechar a conexão!".to_string()),
        Ok(_o) => _o,
    };

    Ok(list)
}