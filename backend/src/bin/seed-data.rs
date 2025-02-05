use chrono::Utc;
use entity::sea_orm_active_enums::{ContentType, Language, Status};
use entity::{category, post_category, post, post_tag, tag};
use fake::Fake;
use fake::faker::lorem::en::Paragraph;
use fake::faker::lorem::en::Word;
use fake::rand::{thread_rng, SeedableRng, rngs::StdRng, prelude::SliceRandom};
use sea_orm::{entity::*, ConnectOptions, TransactionTrait, ActiveValue::Set};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .expect(".cargo/config.toml `DATABASE_URL` variable must be set");

    let opt = ConnectOptions::new(database_url);
    let connection = sea_orm::Database::connect(opt).await?;
    let transaction = connection.begin().await?;

    create_tag(&transaction).await?;
    create_category(&transaction).await?;
    create_post(&transaction).await?;
    create_post_tag(&transaction).await?;
    create_post_category(&transaction).await?;

    transaction.commit().await?;

    Ok(())
}

async fn create_tag(transaction: &sea_orm::DatabaseTransaction) -> Result<(), sea_orm::error::DbErr> {
    let mut rng = StdRng::from_entropy();

    // Generate a random category name
    let name: String = Word().fake_with_rng(&mut rng);
    let slug = name.to_lowercase().replace(" ", "-");
    
    tag::ActiveModel {
        id: Default::default(),
        name: Set(name),
        slug: Set(slug),
        created_at: Set(Utc::now().naive_utc()),
    }.insert(transaction).await?;

    Ok(())
}

async fn create_category(transaction: &sea_orm::DatabaseTransaction) -> Result<(), sea_orm::error::DbErr> {
    let mut rng = StdRng::from_entropy();
    
    // Generate a random category name
    let name: String = Word().fake_with_rng(&mut rng);
    let slug = name.to_lowercase().replace(" ", "-");
    
    category::ActiveModel {
        id: Default::default(),
        name: Set(name),
        slug: Set(slug),
        created_at: Set(Option::from(Utc::now().naive_utc())),
    }.insert(transaction).await?;

    Ok(())
}

async fn create_post(transaction: &sea_orm::DatabaseTransaction) -> Result<(), sea_orm::error::DbErr> {
    let mut rng = StdRng::from_entropy();
    
    // Generate a random post title
    let title: String = Word().fake_with_rng(&mut rng);
    let slug = title.to_lowercase().replace(" ", "-");
    
    // Generate a random post content
    let content: String = Paragraph(3..6).fake_with_rng(&mut rng);

    let content_type = match *["html", "markdown"].choose(&mut rng).unwrap() {
        "html" => Some(ContentType::Html),
        "markdown" => Some(ContentType::Markdown),
        _ => None,
    };

    let language = match *["es", "en"].choose(&mut rng).unwrap() {
        "es" => Some(Language::En),
        "en" => Some(Language::Es),
        _ => None,
    };

    let status = match *["draft", "published"].choose(&mut rng).unwrap() {
        "draft" => Some(Status::Draft),
        "published" => Some(Status::Published),
        _ => None,
    };
    
    let created_at = Option::from(Utc::now().naive_utc());

    post::ActiveModel {
        id: Default::default(),
        title: Set(title),
        slug: Set(slug),
        content: Set(content),
        content_type: Set(content_type),
        language: Set(language),
        featured_image: Default::default(),
        status: Set(status),
        created_at: Set(created_at),
        updated_at: Default::default(),
    }.insert(transaction).await?;

    Ok(())
}

async fn create_post_tag(transaction: &sea_orm::DatabaseTransaction) -> Result<(), sea_orm::error::DbErr> {
    
    post_tag::Entity::delete_many().exec(transaction).await?;
    
    let posts: Vec<post::Model> = post::Entity::find().all(transaction).await?;
    let tags: Vec<tag::Model> = tag::Entity::find().all(transaction).await?;
    
    for post in &posts {
        let random_tag = tags.choose(&mut thread_rng()).unwrap();
        post_tag::ActiveModel {
            id: Default::default(),
            post_id: Set(post.id),
            tag_id: Set(random_tag.id),
        }.insert(transaction).await?;
    }
    
    Ok(())
}

async fn create_post_category(transaction: &sea_orm::DatabaseTransaction) -> Result<(), sea_orm::error::DbErr> {
    
    post_category::Entity::delete_many().exec(transaction).await?;
    
    let posts: Vec<post::Model> = post::Entity::find().all(transaction).await?;
    let categories: Vec<category::Model> = category::Entity::find().all(transaction).await?;

    for post in &posts {
        let random_category = categories.choose(&mut thread_rng()).unwrap();
        post_category::ActiveModel {
            id: Default::default(),
            post_id: Set(post.id),
            category_id: Set(random_category.id),
        }.insert(transaction).await?;
    }
    
    Ok(())
}