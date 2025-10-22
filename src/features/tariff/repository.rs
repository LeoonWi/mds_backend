use std::sync::Arc;

use sqlx::{PgPool, query, query_as};
use tracing::error;

use crate::models::tariff::Tariff;

pub struct Repository {
    pool: Arc<PgPool>,
}

pub fn new(pool: Arc<PgPool>) -> Repository {
    Repository { pool: pool }
}

impl Repository {
    pub async fn add(&self, name: &'static str) -> Result<(), sqlx::Error> {
        query("INSERT INTO tariff (name) VALUE ($1)")
            .bind(name)
            .execute(&*self.pool)
            .await
            .inspect_err(|err| error!("Database error: {err}"))?;

        Ok(())
    }

    pub async fn get(&self) -> Vec<Tariff> {
        let result = query_as::<_, Tariff>("SELECT * FROM tariff")
            .fetch_all(&*self.pool) // *Arc<PgPool> -> PgPool -> &PgPool
            .await
            .unwrap_or_else(|err| {
                error!("Database error: {err}");
                let empty_vec: Vec<Tariff> = Vec::new();
                return empty_vec;
            });

        return result;
    }

    pub async fn get_by_name(&self, name: String) -> Result<Tariff, sqlx::Error> {
        let result = query_as::<_, Tariff>("SELECT * FROM tariff WHERE name = $1")
            .bind(name)
            .fetch_one(&*self.pool)
            .await
            .inspect_err(|e| error!("Database error: {e}"))?;

        Ok(result)
    }
}
