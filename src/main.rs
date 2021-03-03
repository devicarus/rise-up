use actix_web::{post, web, App, HttpServer, HttpResponse, http::StatusCode};
use regex::Regex;

#[post("/rise/{mac}")]
async fn index(web::Path(mac): web::Path<String>) -> HttpResponse {
    let pattern = Regex::new(r"(?:[a-zA-Z0-9]{2}:){5}(?:[a-zA-Z0-9]{2})").unwrap();
    if !pattern.is_match(&mac) {
        return HttpResponse::Ok().status(StatusCode::BAD_REQUEST).finish();
    }

    let wol = wakey::WolPacket::from_string(&mac, ':');
    if wol.send_magic().is_ok() {
        println!("Rising up {}!", mac);
        return HttpResponse::Ok().status(StatusCode::CREATED).finish();
    } else {
        println!("Failed to rise up {}!", mac);
        return HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).finish();
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
