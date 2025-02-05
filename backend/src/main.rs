use axum::{extract::State, routing::get, Json, Router};
use axum::extract::Path;
use entity::post;
use sea_orm::{ConnectOptions, DatabaseConnection, EntityTrait};

#[tokio::main]
async fn main() {

    let database_url = std::env::var("DATABASE_URL")
        .expect(".cargo/config.toml `DATABASE_URL` variable must be set");
    let opt = ConnectOptions::new(database_url);
    let connection = sea_orm::Database::connect(opt).await.expect("Could not connect to database");

    let state = AppState { conn: connection };    
    
    let app = Router::new()
        .route("/", get(|| async { "Hello, World! \nMy personal blog using Rust for backend!" }))
        .route("/posts", get(get_posts))
        .route("/posts/{id}", get(get_post_by_id))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_posts(state: State<AppState>) -> Result<Json<Vec<post::Model>>, (axum::http::StatusCode, String)> {
    match post::Entity::find().all(&state.conn).await {
        Ok(posts) => Ok(Json(posts)),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

async fn get_post_by_id(state: State<AppState>, Path(id): Path<i32>) -> Result<Json<post::Model>, (axum::http::StatusCode, String)> {
    match post::Entity::find_by_id(id).one(&state.conn).await {
        Ok(Some(post)) => Ok(Json(post)),
        Ok(None) => Err((axum::http::StatusCode::NOT_FOUND, "Post not found".to_string())),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}