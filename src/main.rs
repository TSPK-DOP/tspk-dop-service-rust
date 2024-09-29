use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;
mod utils;
mod routes;
use migration::{Migrator, MigratorTrait};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting rusty-chat...");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();  // Если вы используете dotenv для загрузки переменных окружения
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
    })
    .bind((address.as_str(), port))?
    .run()
    .await
}
