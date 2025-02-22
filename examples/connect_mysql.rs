use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::chrono;
use sqlx::{MySql, Pool};
use std::ops::Deref;
use std::sync::Arc;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

// Connect to the database and return a pool of connections with a maximum of 10 connections,
// with a 30 seconds timeout for establishing connections.
pub async fn connect_pool() -> Pool<MySql> {
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect("mysql://root:aa123123@localhost/yx_healthy")
        .await
        .unwrap()
}
#[allow(unused)]
#[derive(Debug, Clone)]
struct AppState {
    inner: Arc<AppStateInner>,
    pool: Pool<MySql>,
}

// AppStateInner is a struct that holds any application-specific state that needs to be shared
// across multiple requests.
impl AppState {
    pub async fn new(pool: Pool<MySql>) -> Self {
        Self {
            inner: Arc::new(AppStateInner {}),
            pool,
        }
    }
}
// Deref allows us to access the inner AppStateInner struct from the AppState struct.
impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct AppStateInner {}

// Custom FormatTime implementation that formats timestamps
// in the format of "2022-01-01T00:00:00.000"
struct LocalTimer;
const fn east8() -> Option<chrono::FixedOffset> {
    chrono::FixedOffset::east_opt(8 * 3600)
}
impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let now = chrono::Utc::now().with_timezone(&east8().unwrap());
        write!(w, "{}", now.format("%FT%T%.3f"))
    }
}

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
    let layer = Layer::new().with_timer(LocalTimer).with_filter(level);
    tracing_subscriber::registry().with(layer).init();

    // Connect to the database and create a pool of connections.
    let pool = connect_pool().await;
    let app_state = AppState::new(pool).await;
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
