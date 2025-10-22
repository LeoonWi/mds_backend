mod handler;
pub mod logic;
pub mod repository;

use std::sync::Arc;

use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn new(pool: Arc<PgPool>) -> Router {
    let repo = Arc::new(repository::new(pool));
    let logic = Arc::new(logic::new(repo));

    Router::new()
        .route("/tariffs", get(handler::get_tariffs))
        .with_state(logic)
}
