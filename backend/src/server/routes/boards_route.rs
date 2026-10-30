use axum::{
    Router, middleware, routing::{delete, get, post, put}
};

use sqlx::PgPool;

use crate::server::{models::auth, services::BoardsService};
use crate::server::BoardsController;
use crate::server::auth_middleware;

pub fn route_init(db_con: &PgPool) -> Router {
    Router::new()
        .route("/boards", get(BoardsController::user_boards))
        .route("/boards", post(BoardsController::add_board))
        .route("/boards/{board_id}", get(BoardsController::provide_board))
        .route("/boards/{board_id}", delete(BoardsController::delete_board))
        .route("/boards/{board_id}", put(BoardsController::update_save))
        .layer(middleware::from_fn(auth_middleware))
        .route("/public/boards/{board_uuid}", get(BoardsController::provide_public_board))
        .with_state(BoardsController::new(BoardsService::new(db_con.clone())))
}
