use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

// Connect to the database and return a pool of connections with a maximum of 10 connections,
// with a 30 seconds timeout for establishing connections.
pub async fn connect_pool() -> Pool<MySql> {
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect("mysql://root:aa123123@localhost/yx_healthy")
        .await
        .unwrap()
}
