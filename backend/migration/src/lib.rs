pub use sea_orm_migration::prelude::*;

mod m20250129_000001_create_table_post;
mod m20250130_042003_create_table_category;
mod m20250131_033230_create_table_tag;
mod m20250131_042256_create_table_post_tag;
mod m20250131_044636_create_table_post_category;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250129_000001_create_table_post::Migration),
            Box::new(m20250130_042003_create_table_category::Migration),
            Box::new(m20250131_033230_create_table_tag::Migration),
            Box::new(m20250131_042256_create_table_post_tag::Migration),
            Box::new(m20250131_044636_create_table_post_category::Migration),
        ]
    }
}
