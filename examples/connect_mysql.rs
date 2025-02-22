use admin_rust::state::AppState;
use admin_rust::{mypool, utils};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber with a filter for the log level.
    // if is_debug is true, set the log level to debug, otherwise set it to info.
    let is_debug = true;
    let level = if is_debug {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };
    let layer = Layer::new()
        .with_timer(utils::LocalTimer)
        .with_filter(level);
    tracing_subscriber::registry().with(layer).init();

    // Connect to the database and create a pool of connections.
    let pool = mypool::connect_pool().await;
    let app_state = AppState::new(pool);
    // Use the pool to execute queries.
    let app_router = Router::new()
        .route("/api/get/user/{id}", get(get_user_handler))
        .with_state(app_state);
    // Start the server.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    // print the listening address
    info!("listening on http://{}", listener.local_addr()?);
    // Start serving requests.
    axum::serve(listener, app_router).await?;
    Ok(())
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
struct User {
    account: String,
    nickname: String,
    level: u32,
}

async fn get_user_handler(
    State(AppState { ref pool, .. }): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    // Execute a query to retrieve all users from the database.
    let query = "SELECT account, nickname, level FROM users where id = ?";
    let res_user: Option<User> = sqlx::query_as(query)
        .bind(id)
        .fetch_optional(pool)
        .await
        .unwrap();
    // If the user is not found, return a 404 error.
    if res_user.is_none() {
        return Json(json!({"code": 404, "message": "user not found"}));
    }
    // Otherwise, return the user information in JSON format.
    let user = res_user.unwrap();
    info!("get user: {:?}", user);
    Json(
        json!({"code": 200, "data": {"account": user.account, "nickname": user.nickname, "level": user.level}}),
    )
}
