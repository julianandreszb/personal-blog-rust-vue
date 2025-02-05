use super::m20250129_000001_create_table_post::Post;
use super::m20250130_042003_create_table_category::Category;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PostCategory::Table)
                    .if_not_exists()
                    .col(pk_auto(PostCategory::Id))
                    .col(ColumnDef::new(PostCategory::PostId).integer().not_null())
                    .col(
                        ColumnDef::new(PostCategory::CategoryId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post_category-post_id")
                            .from(PostCategory::Table, PostCategory::PostId)
                            .to(Post::Table, Post::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post_category-category_id")
                            .from(PostCategory::Table, PostCategory::CategoryId)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostCategory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PostCategory {
    Table,
    Id,
    PostId,
    CategoryId,
}
