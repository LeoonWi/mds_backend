mod handler;
mod logic;
mod repository;

use axum::{Router, routing::post};
use sqlx::PgPool;
use std::sync::Arc;

use crate::models::default_storage::DefaultStorage;

pub fn new(pool: Arc<PgPool>, default_storage: Arc<DefaultStorage>) -> Router {
    let repository = Arc::new(repository::new(pool));
    let logic = Arc::new(logic::new(repository, default_storage));

    return Router::new()
        .route("/employees", post(handler::create_employee))
        .with_state(logic);
}
