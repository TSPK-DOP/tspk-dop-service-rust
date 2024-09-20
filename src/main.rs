    use actix_web::{middleware::Logger, web, App, HttpServer};
use routes::handlers;
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;
use dotenvy::dotenv; 
use env_logger; 
mod utils;
mod routes;
use actix_files::Files;
use migration::{Migrator, MigratorTrait};




#[actix_web::main]
async fn main() -> std::io::Result<()> {

    
    
    let port = (*utils::constants::PORT).clone();
    let address = (*utils::constants::ADDRESS).clone();
    let database_url = (*utils::constants::DATABASE_URL).clone();
    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();
    
    
    HttpServer::new(move ||  { 
        App::new()
        .app_data(web::Data::new( AppState {db: db.clone()}))
        .wrap(Logger::default())
        .configure(routes::home_routes::config)
        .configure(routes::auth_routes::config)
        .service(Files::new("/static", "static").show_files_listing())
        .service(handlers::home_handler::hello)
    })
    .bind((address, port))?
    .run()
    .await
}
    

