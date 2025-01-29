use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")?;
    
    let connection = sea_orm::Database::connect(database_url).await?;
    Migrator::up(&connection, None).await?;
    Ok(())
}