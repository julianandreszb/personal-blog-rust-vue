use std::time::Duration;
use sea_orm::ConnectOptions;
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL").expect(".cargo/config.toml `DATABASE_URL` variable must be set");

    println!("{}", database_url);

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let connection = sea_orm::Database::connect(opt).await?;
    Migrator::up(&connection, None).await?;
    Ok(())
}