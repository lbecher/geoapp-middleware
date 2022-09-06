use actix_web::{HttpResponse, Responder};

pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("
    <!DOCTYPE HTML>
    <html>
    <head>
        <meta http-equiv=\"Content-Type\" content=\"text/html; charset=UTF-8\" />
        <title>GeoApp Middleware</title>
    </head>
    <body>
    </body>
        <h1>Serviço em execução!</h1>
    </html>")
}
