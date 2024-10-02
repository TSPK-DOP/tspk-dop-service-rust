use actix_web::{middleware::Logger, App, HttpServer};
use dotenvy::dotenv;
use sea_orm::Database;
mod utils;
mod routes;
use migration::{Migrator, MigratorTrait};
pub mod config;
mod handlers;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting rusty-chat...");

    std::env::set_var("RUST_LOG", "sea_orm=debug,actix_web=info");
    
    dotenv().ok();  
    env_logger::init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap();
    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await.expect("Не удалось подключиться к базе данных");

    println!("Successfully connected to database");

    Migrator::up(&db, None).await.expect("Ошибка миграции");

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
            .configure(routes::auth_routes::config)
    })
    .bind((address.as_str(), port))?
    .run()
    .await
}