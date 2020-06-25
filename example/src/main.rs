//use actix_web::http::header::ContentType;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
//use std::io::prelude::*;
use std::net::SocketAddr;
extern crate example;

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    let t = example::abc().await;
    HttpResponse::Ok().body(format!("index_page {}", t))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    HttpServer::new(|| {
        App::new()
            .service(index)
//            .route("/", web::get().to(index))
    })
        .bind(&addr)
        .unwrap()
        .run().await
}