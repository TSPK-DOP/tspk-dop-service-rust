use actix_web::web;
use crate::handlers::home_handler;


pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(home_handler::bye);

}