use actix_web::{post, web::{self, Json}, HttpResponse, Responder};
use entity::user;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::utils::{hash_password, verify_password, create_jwt};

#[derive(serde::Deserialize)]   
struct RegisterModel {
    name: String,
    email: String,
    password: String 
}

#[derive(serde::Deserialize)]   
struct LoginModel {
    email: String,
    password: String
}

#[post("/register")]
pub async fn register(db: web::Data<DatabaseConnection>, data: Json<RegisterModel>) -> impl Responder {
    let password_hash = hash_password(&data.password);
    
    let new_user = user::ActiveModel {
        email: Set(data.email.clone()),
        name: Set(data.name.clone()),
        password: Set(password_hash),
        ..Default::default()

    };
    
    let res = user::Entity::insert(new_user).exec(db.get_ref()).await;
    match res {
        Ok(_) => {
            HttpResponse::Ok().json("created")
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("error: {:?}", e))
        }
    }
}


#[post("/login")]
pub async fn login(db: web::Data<DatabaseConnection>, data: Json<LoginModel>) -> impl Responder {
    
    let user = user::Entity::find()
        .filter(
            user::Column::Email.eq(data.email.clone())
        )
        .one(db.get_ref())
        .await
        .unwrap();
    
    
    if let Some(user) = user {
        if verify_password(&data.password, &user.password) {
            let token = create_jwt(&user.email);
            return HttpResponse::Ok().json(token);
        }
    }
    
    HttpResponse::Unauthorized().json("Invalid credentials")
}