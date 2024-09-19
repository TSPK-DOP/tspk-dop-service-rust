use actix_web::{get, post, web};
use super::handlers;


pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/auth")
        );
}