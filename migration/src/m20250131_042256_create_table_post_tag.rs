use sea_orm_migration::{prelude::*, schema::*};
use super::m20250129_000001_create_table_post::Post;
use super::m20250131_033230_create_table_tag::Tag;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PostTag::Table)
                    .if_not_exists()
                    .col(pk_auto(PostTag::Id))
                    .col(
                        ColumnDef::new(PostTag::PostId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PostTag::TagId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post_tag-post_id")
                            .from(PostTag::Table, PostTag::PostId)
                            .to(Post::Table, Post::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post_tag-tag_id")
                            .from(PostTag::Table, PostTag::TagId)
                            .to(Tag::Table, Tag::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostTag::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PostTag {
    Table,
    Id,
    PostId,
    TagId,
}