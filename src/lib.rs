pub mod utils;
pub mod services;
pub mod handlers;
pub mod error;
pub use error::error::AppError;

pub mod entity {
    pub use ::entity::user; 
}