use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::{extract::Path, extract::State, routing::get, Json, Router};
use entity::{category, tag};
use entity::category::CategoryWithName;
use entity::post;
use entity::post::PostApiResponse;
use entity::post::PostPartial;
use entity::post_category;
use entity::post_tag;
use sea_orm::prelude::DateTime;
use sea_orm::{ConnectOptions, DatabaseConnection, EntityTrait, FromQueryResult, JoinType, QuerySelect};
use std::collections::HashMap;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use entity::tag::TagWithName;

#[tokio::main]
async fn main() {
    let backend_port = std::env::var("BACKEND_PORT")
        .expect(".cargo/config.toml `BACKEND_PORT` variable must be set");
    // let frontend_port = std::env::var("FRONTEND_PORT")
    //     .expect(".cargo/config.toml `FRONTEND_PORT` variable must be set");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    tokio::join!(
        // serve(
        //     serve_vue_app(),
        //     frontend_port
        //         .parse()
        //         .expect("FRONTEND_PORT must be a valid port number")
        // ),
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

    let origin = format!("{}:{}", hostname, port)
        .parse()
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
    let hostname_frontend = std::env::var("FRONTEND_HOSTNAME")
        .expect(".cargo/config.toml `FRONTEND_HOSTNAME` variable must be set");

    let port_frontend = std::env::var("FRONTEND_PORT")
        .expect(".cargo/config.toml `FRONTEND_PORT` variable must be set");

    let origin_frontend = format!("{}:{}", hostname_frontend, port_frontend)
        .parse()
        .expect("Invalid frontend hostname and port");

    let hostname = std::env::var("BACKEND_HOSTNAME")
        .expect(".cargo/config.toml `BACKEND  ND_HOSTNAME` variable must be set");

    let port = std::env::var("BACKEND_PORT")
        .expect(".cargo/config.toml `BACKEND_PORT` variable must be set");

    let origin = format!("{}:{}", hostname, port)
        .parse()
        .expect("Invalid frontend hostname and port");

    let origins = [origin, origin_frontend];
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
) -> Result<Json<Vec<PostApiResponse>>, (StatusCode, String)> {

    //region Fetch all posts, categories, and tags
    // 1. Fetch all posts (id and title)
    let posts = post::Entity::find()
        .select_only()
        .column(post::Column::Id)
        .column(post::Column::Title)
        .column(post::Column::Slug)
        .column(post::Column::FeaturedImage)
        .column(post::Column::UpdatedAt)
        .into_model::<PostPartial>()
        .all(&state.conn)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    // 2. Fetch post-category relationships
    let post_categories = post_category::Entity::find()
        .all(&state.conn)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    // 3. Fetch post-tag relationships
    let post_tags = post_tag::Entity::find()
        .all(&state.conn)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    // 4. Fetch all categories
    let categories = category::Entity::find()
        .all(&state.conn)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    // 5. Fetch all tags
    let tags = tag::Entity::find()
        .all(&state.conn)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    //endregion

    //region Create HashMaps for efficient lookups
    let mut category_map: HashMap<i32, Vec<i32>> = HashMap::new();
    
    for post_category in post_categories {
        category_map.entry(post_category.post_id).or_default().push(post_category.category_id);
    }

    let mut tag_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for post_tag in post_tags {
        tag_map.entry(post_tag.post_id).or_default().push(post_tag.tag_id);
    }

    let mut category_by_id: HashMap<i32, CategoryWithName> = HashMap::new();

    for category in categories {
        category_by_id.insert(category.id, CategoryWithName {
            id: category.id,
            name: category.name,
        });
    }

    let mut tag_by_id: HashMap<i32, TagWithName> = HashMap::new();

    for tag in tags {
        tag_by_id.insert(tag.id, TagWithName {
            id: tag.id,
            name: tag.name,
        });
    }
    //endregion

    //region Build the final result
    let mut result: Vec<PostApiResponse> = Vec::with_capacity(posts.len());
    for post in posts {
        let categories = category_map
            .get(&post.id)
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|category_id| category_by_id.get(category_id))
            .cloned()
            .collect();

        let tags = tag_map
            .get(&post.id)
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|tag_id| tag_by_id.get(tag_id))
            .cloned()
            .collect();

        result.push(PostApiResponse {
            id: post.id,
            title: post.title,
            slug: post.slug,
            featured_image: post.featured_image,
            updated_at: post.updated_at,
            categories,
            tags,
        });
    }
    //endregion

    Ok(Json(result))
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
