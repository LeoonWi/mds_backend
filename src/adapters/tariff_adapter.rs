use std::sync::Arc;

use sqlx::{PgPool, query, query_as};
use tracing::error;

use crate::{
    application::tariff::TariffAdapter,
    models::{error::AppError, tariff::Tariff},
};

pub struct TariffRepository {
    pool: Arc<PgPool>,
}

impl TariffRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        TariffRepository { pool }
    }
}

impl TariffAdapter for TariffRepository {
    async fn save(&self, name: String) -> Result<(), AppError> {
        query("INSERT INTO tariff (name) VALUE ($1)")
            .bind(&name)
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::Conflict
            })?;

        Ok(())
    }

    async fn get(&self) -> Result<Vec<Tariff>, AppError> {
        query_as::<_, Tariff>("SELECT * FROM tariff")
            .fetch_all(&*self.pool) // *Arc<PgPool> -> PgPool -> &PgPool
            .await
            .map_err(|err| {
                error!("Database error: {err}");
                AppError::NotFound
            })
    }

    async fn get_by_name(&self, name: String) -> Result<Tariff, AppError> {
        query_as::<_, Tariff>("SELECT * FROM tariff WHERE name = $1")
            .bind(&name)
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| {
                error!("Database error: {err}");
                AppError::NotFound
            })
    }
}
