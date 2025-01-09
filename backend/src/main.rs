use actix_web::{web, App, HttpResponse, HttpServer};
mod controllers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(routes::query::handle_query) // Use `.service` for `#[post]` handlers
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
