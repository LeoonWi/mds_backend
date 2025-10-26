use std::sync::Arc;

use sqlx::{PgPool, query, query_as};
use tracing::error;

use crate::application::user::UserAdapter;
use crate::models::user::UserFlat;
use crate::models::error::AppError;

pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        UserRepository { pool }
    }
}

impl UserAdapter for UserRepository {
    async fn save(
        &self,
        name: String,
        last_name: String,
        middle_name: Option<String>,
        email: String,
        phone: String,
        tariff: String,
        inn: Option<String>,
        snils: Option<String>,
        password: String,
    ) -> Result<(), AppError> {
        query(
            "INSERT INTO \"user\"
            (name, last_name, middle_name, email, phone, password, tariff, inn, snils)
            VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(name)
        .bind(last_name)
        .bind(middle_name)
        .bind(email)
        .bind(phone)
        .bind(password)
        .bind(tariff)
        .bind(inn)
        .bind(snils)
        .execute(&*self.pool)
        .await
        .map_err(|e| {
            error!("Database error: {e}");
            AppError::Conflict
        })?;

        Ok(())
    }

    async fn get(&self) -> Result<Vec<UserFlat>, AppError> {
        query_as::<_, UserFlat>(
            "SELECT u.*, t.name as tariff_name, t.created_at as tariff_created_at, t.updated_at as tariff_updated_at 
             FROM \"user\" u 
             JOIN tariff t ON u.tariff = t.name"
        )
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::InternalServerError
            })
    }

    async fn get_by_email(&self, email: String) -> Result<UserFlat, AppError> {
        query_as::<_, UserFlat>(
            "SELECT u.*, t.name as tariff_name, t.created_at as tariff_created_at, t.updated_at as tariff_updated_at 
             FROM \"user\" u 
             JOIN tariff t ON u.tariff = t.name 
             WHERE u.email = $1"
        )
            .bind(email)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::NotFound
            })
    }

    async fn get_by_phone(&self, phone: String) -> Result<UserFlat, AppError> {
        query_as::<_, UserFlat>(
            "SELECT u.*, t.name as tariff_name, t.created_at as tariff_created_at, t.updated_at as tariff_updated_at 
             FROM \"user\" u 
             JOIN tariff t ON u.tariff = t.name 
             WHERE u.phone = $1"
        )
            .bind(phone)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::NotFound
            })
    }
}