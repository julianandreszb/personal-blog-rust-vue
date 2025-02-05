use axum::http::header::CONTENT_TYPE;
use axum::{extract::Path, extract::State, routing::get, Json, Router};
use entity::post;
use sea_orm::{ConnectOptions, DatabaseConnection, EntityTrait};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let backend_port = std::env::var("BACKEND_PORT")
        .expect(".cargo/config.toml `BACKEND_PORT` variable must be set");
    let frontend_port = std::env::var("FRONTEND_PORT")
        .expect(".cargo/config.toml `FRONTEND_PORT` variable must be set");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tokio::join!(
        serve(
            serve_vue_app(),
            frontend_port
                .parse()
                .expect("FRONTEND_PORT must be a valid port number")
        ),
        serve(
            serve_api().await,
            backend_port
                .parse()
                .expect("BACKEND_PORT must be a valid port number")
        )
    );
}

fn serve_vue_app() -> Router {
    let hostname = std::env::var("FRONTEND_HOSTNAME")
        .expect(".cargo/config.toml `FRONTEND_HOSTNAME` variable must be set");
    let port = std::env::var("FRONTEND_PORT")
        .expect(".cargo/config.toml `FRONTEND_PORT` variable must be set");
    let origin = format!("{}:{}", hostname, port).parse()
        .expect("Invalid frontend hostname and port");
    let origins = [origin];

    Router::new()
        .nest_service("/app", ServeDir::new("../frontend/dist"))
        .layer(
            CorsLayer::new()
                .allow_origin(origins)
                .allow_headers([CONTENT_TYPE])
                .allow_methods([axum::http::Method::GET]),
        )
}

async fn serve_api() -> Router {
    let hostname = std::env::var("BACKEND_HOSTNAME")
        .expect(".cargo/config.toml `BACKEND_HOSTNAME` variable must be set");
    let port = std::env::var("BACKEND_PORT")
        .expect(".cargo/config.toml `BACKEND_PORT` variable must be set");
    let origin = format!("{}:{}", hostname, port).parse()
        .expect("Invalid frontend hostname and port");
    let origins = [origin];

    let database_url = std::env::var("DATABASE_URL")
        .expect(".cargo/config.toml `DATABASE_URL` variable must be set");
    let opt = ConnectOptions::new(database_url);
    let connection = sea_orm::Database::connect(opt)
        .await
        .expect("Could not connect to database");

    let state = AppState { conn: connection };
    let api_router = Router::new()
        .route("/posts", get(get_posts))
        .route("/posts/{id}", get(get_post_by_id))
        .layer(
            CorsLayer::new()
                .allow_origin(origins)
                .allow_headers([CONTENT_TYPE])
                .allow_methods([axum::http::Method::GET]),
        )
        .with_state(state);

    Router::new().nest("/api", api_router)
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Could not bind to port");
    tracing::debug!(
        "listening on {}",
        listener.local_addr().expect("Could not get local address")
    );
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .expect("Could not start server");
}

async fn get_posts(
    state: State<AppState>,
) -> Result<Json<Vec<post::Model>>, (axum::http::StatusCode, String)> {
    match post::Entity::find().all(&state.conn).await {
        Ok(posts) => Ok(Json(posts)),
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
        )),
    }
}

async fn get_post_by_id(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<post::Model>, (axum::http::StatusCode, String)> {
    match post::Entity::find_by_id(id).one(&state.conn).await {
        Ok(Some(post)) => Ok(Json(post)),
        Ok(None) => Err((
            axum::http::StatusCode::NOT_FOUND,
            "Post not found".to_string(),
        )),
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
        )),
    }
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}
