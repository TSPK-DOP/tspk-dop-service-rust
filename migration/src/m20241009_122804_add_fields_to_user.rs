use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(
                        ColumnDef::new(User::Surname)
                                .string()
                                .not_null()
                                .default(""),
                    )
                    .add_column(
                        ColumnDef::new(User::Group)
                            .string()
                            .null()
                    )
                    .add_column(
                        ColumnDef::new(User::Role)
                            .string()
                            .not_null()
                            .default("Student")
                    )
                    .add_column(
                        ColumnDef::new(User::AdditionalContact)
                            .string()
                            .null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .alter_table(
                    Table::alter()
                        .table(User::Table)
                        .drop_column(User::Surname)
                        .drop_column(User::Group)
                        .drop_column(User::Role)
                        .drop_column(User::AdditionalContact)
                        .to_owned(),
                )
                .await
        }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Surname,
    Group,
    Role,
    AdditionalContact,
}
