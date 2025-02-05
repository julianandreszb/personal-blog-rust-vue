use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Category::Name)
                            .string()
                            .not_null()
                            .string_len(255),
                    )
                    .col(
                        ColumnDef::new(Category::Slug)
                            .string()
                            .not_null()
                            .string_len(255)
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Category::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Category {
    Table,
    Id,
    Name,
    Slug,
    CreatedAt,
}
