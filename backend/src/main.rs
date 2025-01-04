use actix_web::{web, App, HttpResponse, HttpServer};
use serde_json::json;

async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "item_id": 123 }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
