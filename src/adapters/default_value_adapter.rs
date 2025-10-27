use std::sync::Arc;

use sqlx::{PgPool, query, query_as};
use tracing::error;

use crate::application::default_value::DefaultValueAdapter;
use crate::models::error::AppError;
use crate::models::role::Role;
use crate::models::tariff::Tariff;

pub struct DefaultValueRepository {
    pool: Arc<PgPool>,
}

impl DefaultValueRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        DefaultValueRepository { pool }
    }
}

impl DefaultValueAdapter for DefaultValueRepository {
    async fn save(&self, tariff: String, role: String) -> Result<(), AppError> {
        query("INSERT INTO default_value (id, tariff, role) VALUES ($1, $2, $3)")
            .bind(1 as i32)
            .bind(tariff)
            .bind(role)
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::Conflict
            })?;

        Ok(())
    }

    async fn save_tariff(&self, tariff: String) -> Result<(), AppError> {
        query("UPDATE default_value SET tariff = $1 WHERE id = 1")
            .bind(tariff)
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::BadRequest(e.to_string())
            })?;

        Ok(())
    }

    async fn save_role(&self, role: String) -> Result<(), AppError> {
        query("UPDATE default_value SET role = $1 WHERE id = 1")
            .bind(role)
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::BadRequest(e.to_string())
            })?;

        Ok(())
    }

    async fn get_tariff(&self) -> Result<Tariff, AppError> {
        query_as::<_, Tariff>(
            "SELECT * FROM tariff WHERE name = (SELECT tariff FROM default_value WHERE id = 1)",
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| {
            error!("Database error: {e}");
            AppError::NotFound
        })
    }

    async fn get_role(&self) -> Result<Role, AppError> {
        query_as::<_, Role>(
            "SELECT * FROM role WHERE name = (SELECT role FROM default_value WHERE id = 1)",
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| {
            error!("Database error: {e}");
            AppError::NotFound
        })
    }
}
