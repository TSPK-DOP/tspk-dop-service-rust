use actix_web::{post, delete, put, web::{self, JsonBody}, HttpResponse, Responder};

use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set};
use entity::user::{self, ActiveModel, UserRole};
use serde::Deserialize;
use crate::utils::access_control::check_access_level; 
use sea_orm::ActiveValue;

#[derive(Deserialize)]
pub struct AdditionalContactRequest {
    pub additional_contact: Option<String>,
    pub user_role_id: i32,
}

fn convert_role(role_id: &i32) -> Option<UserRole> {
    match role_id {
        1 => Some(UserRole::Student),
        2 => Some(UserRole::Teacher),
        3 => Some(UserRole::Admin),
        _ => None, 
    }
}

#[post("/user/{user_id}/add-contact")]
pub async fn add_additional_contact(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i32>,
    req: web::Json<AdditionalContactRequest>, 
) -> impl Responder{
    
    let user_role = match convert_role(&req.user_role_id) {
        Some(role) => role,
        None => return HttpResponse::BadRequest().body("Недопустимая роль"),
    };
    
    if !check_access_level(user_role, 2).is_ok() {
            return HttpResponse::Forbidden().body("Недостаточно прав для выполнения операции");
        }
        
    let user_result = user::Entity::find_by_id(user_id.into_inner())
        .one(db.get_ref())
        .await;
    match user_result {
        Ok(Some(user)) => {
            let mut user: user::ActiveModel = user.into(); 
            if matches!(
                user.additional_contact,
                ActiveValue::Set(Some(_)) | ActiveValue::Unchanged(Some(_))
            ) {
                return HttpResponse::BadRequest().body("Дополнительный контакт уже существует");
            }

            user.additional_contact = Set(Some(req.additional_contact.clone().unwrap()));
            match user.update(db.get_ref()).await {
                Ok(update_user) => HttpResponse::Ok().body(format!(
                    "Доп. контакт добавлен"
                )),
                Err(_) =>HttpResponse::InternalServerError().body("Ошибка при обновлении"),
                
            }
        },
        Ok(None) => HttpResponse::NotFound().body("Пользователь не найден"), 
        Err(_) => HttpResponse::InternalServerError().body("Ошибка при выполнении запроса к базе данных"), 
    }
    
}

#[put("/user/{user_id}/update-contact")]
pub async fn update_additional_contact(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i32>,
    req: web::Json<AdditionalContactRequest>, 
) -> impl Responder{
    
    let user_role = match convert_role(&req.user_role_id) {
        Some(role) => role,
        None => return HttpResponse::BadRequest().body("Недопустимая роль"),
    };
    
    if !check_access_level(user_role, 2).is_ok() {
            return HttpResponse::Forbidden().body("Недостаточно прав для выполнения операции");
        }
        
    let user_result = user::Entity::find_by_id(user_id.into_inner())
        .one(db.get_ref())
        .await;
    match user_result {
        Ok(Some(user)) => {
            let mut user: user::ActiveModel = user.into(); 
            if !user.additional_contact.into_value().is_some() {
                return HttpResponse::BadRequest().body("Контакт нужно сначала добавить"); 
            }
            user.additional_contact = Set(Some(req.additional_contact.clone().unwrap()));
            match user.update(db.get_ref()).await {
                Ok(update_user) => HttpResponse::Ok().body(format!(
                    "Доп. контакт обновлен"
                )),
                Err(_) =>HttpResponse::InternalServerError().body("Ошибка при обновлении"),
                
            }
        },
        Ok(None) => HttpResponse::NotFound().body("Пользователь не найден"), 
        Err(_) => HttpResponse::InternalServerError().body("Ошибка при выполнении запроса к базе данных"), 
    }
    
}


#[delete("/user/{user_id}/delete-contact")]
pub async fn delete_additional_contact(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i32>,
    req: web::Json<AdditionalContactRequest>, 
) -> impl Responder{
    let user_role = match convert_role(&req.user_role_id) {
        Some(role) => role,
        None => return HttpResponse::BadRequest().body("Недопустимая роль"),
    };
    if !check_access_level(user_role, 2).is_ok() {
            return HttpResponse::Forbidden().body("Недостаточно прав для выполнения операции");
        };
        
    let user_result = user::Entity::find_by_id(user_id.into_inner())
        .one(db.get_ref())
        .await;
    
    match user_result {
        Ok(Some(user)) => {
            let mut user: ActiveModel = user.into();
            
            if user.additional_contact.clone().into_value().is_none() {
                return HttpResponse::BadRequest().body("Дополнительный контакт не существует");
            }

            user.additional_contact = Set(None);

            match user.update(db.get_ref()).await {
                Ok(_) => HttpResponse::Ok().body("Дополнительный контакт удален"),
                Err(_) => HttpResponse::InternalServerError().body("Ошибка при удалении контакта"),
            }
        },
        Ok(None) => HttpResponse::NotFound().body("Пользователь не найден"),
        Err(_) => HttpResponse::InternalServerError().body("Ошибка при выполнении запроса к базе данных"),
    }
}