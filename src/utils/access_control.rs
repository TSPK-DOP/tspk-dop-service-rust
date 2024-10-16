use entity::user::UserRole;

pub trait AccessLevel {
    fn as_access_level(&self) -> i32;
}

impl AccessLevel for UserRole {
    fn as_access_level(&self) -> i32 {
        match self {
            UserRole::Admin => 3,    // Уровень доступа 3 - Администратор
            UserRole::Teacher => 2,  // Уровень доступа 2 - Преподаватель
            UserRole::Student => 1,  // Уровень доступа 1 - Студент
        }
    }
}

pub fn check_access_level(user_role: UserRole, required_level: i32) -> Result<(), String> {
    if user_role.as_access_level() >= required_level {
        Ok(()) 
    } else {
        Err("Недостаточно прав для выполнения операции".to_string())
    }
}
