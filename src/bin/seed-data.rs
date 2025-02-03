use chrono::Utc;
use entity::tag;
use sea_orm::{entity::*, ConnectOptions, TransactionTrait};
use sea_orm::ActiveValue::Set;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .expect(".cargo/config.toml `DATABASE_URL` variable must be set");

    let opt = ConnectOptions::new(database_url);
    let connection = sea_orm::Database::connect(opt).await?;
    let transaction = connection.begin().await?;

    tag::ActiveModel {
        id: Default::default(),
        name: Set("rust".to_owned()),
        slug: Set("rust".to_owned()),
        created_at: Set(Utc::now().naive_utc()),
    }
    .insert(&transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}
