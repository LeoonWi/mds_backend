use std::sync::Arc;

use sqlx::{PgPool, query, query_as};
use tracing::error;

use crate::models::default_storage::DefaultStorage;

pub struct Repository {
    pool: Arc<PgPool>,
}

pub fn new(pool: Arc<PgPool>) -> Repository {
    Repository { pool: pool }
}

impl Repository {
    pub async fn add(&self, role: String, tariff: String) -> Result<(), sqlx::Error> {
        query("INSERT INTO default_value (tariff, role) VALUE ($1, $2)")
            .bind(tariff)
            .bind(role)
            .execute(&*self.pool)
            .await
            .inspect_err(|e| error!("Database error: {e}"))?;

        Ok(())
    }

    pub async fn get(&self) -> Result<DefaultStorage, sqlx::Error> {
        let result = query_as::<_, DefaultStorage>("SELECT * FROM default_value")
            .fetch_one(&*self.pool)
            .await
            .inspect_err(|e| error!("Database error: {e}"))?;

        Ok(result)
    }
}
