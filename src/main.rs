use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use routes::handlers;
use std::env; 
use dotenvy::dotenv; 
use env_logger; 
mod utils;
mod routes;
use actix_files::Files;




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();
    
    
    let port = (*utils::constants::PORT).clone();
    let address = (*utils::constants::ADDRESS).clone();

    HttpServer::new(||  { 
        App::new()
        .wrap(Logger::default())
        .configure(routes::home_routes::config)
        .service(Files::new("/static", "static").show_files_listing())
        .service(handlers::home_handler::hello)
    })
    .bind((address, port))?
    .run()
    .await
}