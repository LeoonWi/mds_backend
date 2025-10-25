use std::sync::Arc;

use sqlx::{PgPool, query, query_as};
use tracing::error;

use crate::application::role::RoleAdapter;
use crate::models::error::AppError;
use crate::models::role::Role;

pub struct RoleRepository {
    pool: Arc<PgPool>,
}

impl RoleRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        RoleRepository { pool }
    }
}

impl RoleAdapter for RoleRepository {
    async fn save(&self, name: String) -> Result<(), AppError> {
        let _ = query("INSERT INTO role (name) VALUE ($1)")
            .bind(&name)
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::Conflict
            })?;

        Ok(())
    }

    async fn get(&self) -> Result<Vec<Role>, AppError> {
        let result = query_as::<_, Role>("SELECT * FROM role")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::NotFound
            });

        return result;
    }

    async fn get_by_name(&self, name: String) -> Result<Role, AppError> {
        let result = query_as::<_, Role>("SELECT * FROM role WHERE name = $1")
            .bind(&name)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::NotFound
            });

        return result;
    }
}
