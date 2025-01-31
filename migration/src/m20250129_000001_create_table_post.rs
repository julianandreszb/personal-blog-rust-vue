use sea_orm_migration::{prelude::*};
use crate::sea_orm::{EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Post::Title)
                            .string()
                            .not_null()
                            .string_len(255),
                    )
                    .col(
                        ColumnDef::new(Post::Slug)
                            .string()
                            .not_null()
                            .string_len(255)
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Post::Content)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Post::ContentType)
                            .enumeration(Alias::new("content_type"), ContentType::iter())
                            .default("markdown"),
                    )
                    .col(
                        ColumnDef::new(Post::Language)
                            .string()
                            .string_len(50),
                    )
                    .col(
                        ColumnDef::new(Post::FeaturedImage)
                            .string()
                            .string_len(255),
                    )
                    .col(
                        ColumnDef::new(Post::Status)
                            .enumeration(Alias::new("status"), Status::iter())
                            .default("draft"),
                    )
                    .col(
                        ColumnDef::new(Post::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Post::UpdatedAt)
                            .date_time()
                            .extra("ON UPDATE CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Post {
    Table,
    Id,
    Title,
    Slug,
    Content,
    ContentType,
    Language,
    FeaturedImage,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden, EnumIter)]
pub enum ContentType {
    #[iden = "html"]
    Html,
    #[iden = "markdown"]
    Markdown,
}

#[derive(Iden, EnumIter)]
pub enum Status {
    #[iden = "draft"]
    Draft,
    #[iden = "published"]
    Published,
}