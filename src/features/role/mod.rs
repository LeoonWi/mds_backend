mod handler;
pub mod logic;
pub mod repository;

use axum::{Router, routing::get};
use std::sync::Arc;

pub fn new(pool: Arc<sqlx::PgPool>) -> Router {
    let repo = Arc::new(repository::new(pool));
    let logic = Arc::new(logic::new(repo));

    return Router::new()
        .route("/roles", get(handler::get_roles))
        .with_state(logic);
}
