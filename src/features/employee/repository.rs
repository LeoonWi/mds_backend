use std::sync::Arc;

use sqlx::PgPool;

pub struct Repository {
    pool: Arc<PgPool>,
}

pub fn new(pool: Arc<PgPool>) -> Repository {
    Repository { pool: pool }
}
