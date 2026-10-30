mod pcad_parser;
mod models;
mod server;

use crate::server::{auth_route_init, board_route_init, static_handler};
use axum::Router;
use sqlx::PgPool;
use tokio::net::TcpListener;
use dotenv;


#[tokio::main]
async fn main() {
    let db_url = dotenv::var("DATABASE_URL")
        .expect("Database address needed");

    let db_con = PgPool::connect(&db_url)
        .await
        .unwrap_or_else(|e| panic!("{}", e));

    let app = Router::new()
        .nest("/api", auth_route_init(&db_con))
        .nest("/api", board_route_init(&db_con))
        .fallback(static_handler);

    let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


