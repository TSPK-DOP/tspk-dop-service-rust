use actix_web::web;
use super::handlers;


pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/home")
            .service(handlers::home_handler::hello)
            .service(handlers::home_handler::bye)
        );
}