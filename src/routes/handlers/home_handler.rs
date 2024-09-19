use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::NamedFile;

use crate::utils::api_response;

#[get("/")]
pub async fn hello() -> impl Responder {
    NamedFile::open("static/index.html")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[get("/goodbye")]
pub async fn bye() -> impl Responder {
    api_response::ApiResponse::new(42342, "BYE".to_string())
}