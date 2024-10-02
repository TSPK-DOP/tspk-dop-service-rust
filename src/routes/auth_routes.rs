use actix_web::web;
use crate::handlers::auth_handler;


pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(auth_handler::login)
        .service(auth_handler::register);


}