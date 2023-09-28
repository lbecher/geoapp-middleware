use tokio::task;
use actix_web::{post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use postgres::{Client, NoTls};
use geojson::{FeatureCollection, Feature};
use std::io::Write;

#[path = "../common/getpath.rs"]
mod getpath;
use getpath::{getpath, Conn};

#[path = "../common/getbody.rs"]
mod getbody;
use getbody::getbody;

#[derive(Serialize, Deserialize)]
struct Req {
    conn: Conn,
    table: String,
}

#[post("/getgeojson")]
async fn getgeojson(payload: web::Payload) -> Result<HttpResponse, Error> {
    let body: web::BytesMut = match getbody(payload).await {
        Ok(body) => body,
        Err(e) => return Err(e)
    };
    
    let req = serde_json::from_slice::<Req>(&body)?;

    let path = getpath(req.conn);

    let geojson_feature_collection = task::spawn_blocking(move ||
        pg_getgeojson(path.as_str(), req.table.as_str())
    ).await.unwrap().unwrap();

    let mut res = HttpResponse::Ok();
    let mut body = Vec::new();

    // Serializa a FeatureCollection em um vetor de bytes
    let geojson_bytes = serde_json::to_vec(&geojson_feature_collection).map_err(|e| {
        eprintln!("Erro ao serializar GeoJSON: {:?}", e);
    }).unwrap();

    // Define o Content-Type do response como application/json
    res.content_type("application/octet-stream");

    // Define o corpo da resposta como os bytes serializados
    body.extend_from_slice(&geojson_bytes);
    return Ok(res.body(body));
}

fn pg_getgeojson(path: &str, table: &str) -> Result<FeatureCollection, String> {
    let mut client = match Client::connect(path, NoTls) {
        Err(_e) => return Err("Erro ao conectar ao banco de dados!".to_string()),
        Ok(client) => client,
    };

    let mut geojson_feature_collection = FeatureCollection {
        bbox: None,
        features: vec![],
        foreign_members: None
    };

    let query = format!("SELECT ST_AsGeoJSON(ST_Transform(geom, 4326)) FROM public.{}", table);

    for row in client.query(query.as_str(), &[]).unwrap() {
        let geometry = row.get(0);

        let geojson_feature = Feature {
            bbox: None,
            id: None,
            geometry: serde_json::from_str(geometry).unwrap(),
            properties: None,
            foreign_members: None
        };

        geojson_feature_collection.features.push(geojson_feature);
    }

    match client.close()  {
        Err(_e) => return Err("Não foi possível fechar a conexão!".to_string()),
        Ok(_o) => _o,
    };

    Ok(geojson_feature_collection)
}