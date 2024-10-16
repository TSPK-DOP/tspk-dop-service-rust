pub use sea_orm_migration::prelude::*;

mod m20240914_110830_create_user_table;
mod m20241009_122804_add_fields_to_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240914_110830_create_user_table::Migration),
            Box::new(m20241009_122804_add_fields_to_user::Migration),
        ]
    }
}
