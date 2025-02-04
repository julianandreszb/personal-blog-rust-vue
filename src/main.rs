use axum::{extract::State, routing::get, Json, Router};
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
        .route("/", get(|| async { "Hello, World!" }))
        .route("/posts", get(get_posts))
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

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}