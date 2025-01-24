use actix_web::{get, post, HttpResponse, Responder};
use crate::utils::api_response;



#[post("/")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[get("/goodbye")]
pub async fn bye() -> impl Responder {
    api_response::ApiResponse::new(200, "Hello".to_string())
}