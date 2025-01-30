pub use sea_orm_migration::prelude::*;

mod m20250129_000001_create_table_post;
mod m20250130_042003_create_table_category;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250129_000001_create_table_post::Migration),
            Box::new(m20250130_042003_create_table_category::Migration),
        ]
    }
}
