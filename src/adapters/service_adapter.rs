use std::sync::Arc;

use sqlx::{PgPool, query, query_as};

use crate::application::service::ServiceAdapter;
use crate::models::error::AppError;
use crate::models::service::Service;

pub struct ServiceRepository {
    pool: Arc<PgPool>,
}

impl ServiceRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        ServiceRepository { pool }
    }
}

impl ServiceAdapter for ServiceRepository {
    async fn save(&self, name: String) -> Result<(), AppError> {
        query("INSERT INTO service (name) VALUES ($1)")
            .bind(name)
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Database error: {e}");
                AppError::InternalServerError
            })?;

        Ok(())
    }

    async fn get(&self) -> Result<Vec<Service>, AppError> {
        query_as("SELECT * FROM service")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Database error: {e}");
                AppError::InternalServerError
            })
    }

    async fn get_by_name(&self, name: String) -> Result<Service, AppError> {
        query_as::<_, Service>("SELECT * FROM service WHERE name = $1")
            .bind(name)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Database error: {e}");
                AppError::InternalServerError
            })
    }

    async fn delete(&self, name: String) -> Result<(), AppError> {
        let row = query("DELETE FROM service WHERE name = $1")
            .bind(name)
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Database error: {e}");
                AppError::InternalServerError
            })?;

        if row.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
