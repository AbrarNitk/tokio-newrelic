//use actix_web::http::header::ContentType;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
//use std::io::prelude::*;
use std::net::SocketAddr;

extern crate example;

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    println!("path: {}", _req.path());
    let t = example::newrelic_transaction_function().await;
    HttpResponse::Ok().body(format!("index_page {:?}", t))
}

#[get("/a")]
async fn index1(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body(format!("index_page1"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    HttpServer::new(|| App::new().service(index).service(index1))
        .bind(&addr)
        .unwrap()
        .run()
        .await
}
