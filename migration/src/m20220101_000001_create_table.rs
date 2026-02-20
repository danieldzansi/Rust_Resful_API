use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {

    // Runs when we APPLY the migration
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Todos::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Todos::Description)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Todos::Completed)
                            .boolean()
                            .not_null()
                            .default(false)
                    )
                    .col(
                        ColumnDef::new(Todos::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT now()")
                    )
                    .to_owned(),
            )
            .await
    }

    // Runs when we ROLLBACK the migration
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Todos::Table)
                    .to_owned()
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Todos {
    Table,
    Id,
    Description,
    Completed,
    CreatedAt,
}