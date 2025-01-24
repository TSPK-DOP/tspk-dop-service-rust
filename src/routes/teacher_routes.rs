use actix_web::web;
use crate::handlers::{auth_handler, teacher::contact_handlers};


pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(contact_handlers::add_additional_contact)
        .service(contact_handlers::update_additional_contact)
        .service(contact_handlers::delete_additional_contact);


}