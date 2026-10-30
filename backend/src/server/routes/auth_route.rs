use axum::{
    routing::{get, post},
    Router
};

use sqlx::PgPool;

use crate::server::services::AuthService;
use crate::server::controllers::AuthController;

pub fn route_init(db_con: &PgPool) -> Router {
    Router::new()
        .route("/register", post(AuthController::register))
        .route("/login", post(AuthController::login))
        .route("/logout", get(AuthController::logout))
        .with_state(AuthController::new(
            AuthService::new(db_con.clone())
        ))
}


