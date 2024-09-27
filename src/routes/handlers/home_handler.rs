use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::NamedFile;

use crate::utils::api_response;



#[post("/")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[get("/goodbye")]
pub async fn bye() -> impl Responder {
    api_response::ApiResponse::new(200, "BYE".to_string())
}