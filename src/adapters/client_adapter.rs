use std::sync::Arc;

use sqlx::{PgPool, QueryBuilder, query, query_as};
use tracing::error;

use crate::application::client::ClientAdapter;
use crate::models::client::{ClientFlat, FilterClient};
use crate::models::error::AppError;

pub struct ClientRepository {
    pool: Arc<PgPool>,
}

impl ClientRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        ClientRepository { pool }
    }
}

impl ClientAdapter for ClientRepository {
    async fn save(
        &self,
        name: String,
        last_name: String,
        middle_name: Option<String>,
        email: String,
        phone: String,
        password: String,
        tariff: String,
        inn: Option<String>,
        snils: Option<String>,
    ) -> Result<(), AppError> {
        query(
            "INSERT INTO \"user\" (name, last_name, middle_name, email, phone, password, tariff, inn, snils)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
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

    async fn get(&self, filter: FilterClient) -> Result<Vec<ClientFlat>, AppError> {
        let mut query: QueryBuilder<'_, sqlx::Postgres> = QueryBuilder::new(
            "SELECT
                c.id,
                c.name,
                c.last_name,
                c.middle_name,
                c.email,
                c.phone,
                c.password,
                c.inn,
                c.snils,
                c.created_at,
                c.updated_at,
                t.name AS tariff_name,
                t.created_at AS tariff_created_at,
                t.updated_at AS tariff_updated_at
            FROM \"user\" AS c
            INNER JOIN tariff AS t ON t.name = c.tariff
            WHERE 1=1",
        );

        if let Some(id) = filter.id {
            query.push(" AND c.id = ").push_bind(id);
        }

        if let Some(name) = filter.name {
            query.push(" AND c.name = ").push_bind(name);
        }

        if let Some(last_name) = filter.last_name {
            query.push(" AND c.last_name = ").push_bind(last_name);
        }

        if let Some(middle_name) = filter.middle_name {
            query.push(" AND c.middle_name = ").push_bind(middle_name);
        }

        if let Some(email) = filter.email {
            query.push(" AND c.email = ").push_bind(email);
        }

        if let Some(phone) = filter.phone {
            query.push(" AND c.phone = ").push_bind(phone);
        }

        if let Some(tariff) = filter.tariff {
            query.push(" AND c.tariff = ").push_bind(tariff);
        }

        query
            .build_query_as::<ClientFlat>()
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::Conflict
            })
    }

    #[allow(dead_code)]
    async fn get_by_email(&self, email: String) -> Result<ClientFlat, AppError> {
        query_as::<_, ClientFlat>(
            "SELECT
                c.id,
                c.name,
                c.last_name,
                c.middle_name,
                c.email,
                c.phone,
                c.password,
                c.inn,
                c.snils,
                c.created_at,
                c.updated_at,
                t.name AS tariff_name,
                t.created_at AS tariff_created_at,
                t.updated_at AS tariff_updated_at
            FROM \"user\" AS c
            INNER JOIN tariff AS t ON t.name = c.tariff
            WHERE c.email = $1",
        )
        .bind(email)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| {
            error!("Database error: {e}");
            AppError::NotFound
        })
    }

    async fn delete(&self, email: String) -> Result<(), AppError> {
        let result = query("DELETE FROM \"user\" WHERE email = $1")
            .bind(email)
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::InternalServerError
            })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
