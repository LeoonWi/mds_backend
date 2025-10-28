use std::sync::Arc;

use chrono::Utc;
use sqlx::{PgPool, Postgres, QueryBuilder, query, query_as};
use tracing::error;

use crate::application::employee::EmployeeAdapter;
use crate::models::employee::{EmployeeFlat, FilterEmployee};
use crate::models::error::AppError;

pub struct EmployeeRepository {
    pool: Arc<PgPool>,
}

impl EmployeeRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        EmployeeRepository { pool }
    }
}

impl EmployeeAdapter for EmployeeRepository {
    async fn save(
        &self,
        name: String,
        last_name: String,
        middle_name: Option<String>,
        email: String,
        role: String,
        dismissed: bool,
        password: String,
    ) -> Result<(), AppError> {
        query(
            "INSERT INTO employee (name, last_name, middle_name, email, password, role, dismissed)
            VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(name)
        .bind(last_name)
        .bind(middle_name)
        .bind(email)
        .bind(password)
        .bind(role)
        .bind(dismissed)
        .execute(&*self.pool)
        .await
        .map_err(|e| {
            error!("Database error: {e}");
            AppError::Conflict
        })?;

        Ok(())
    }

    async fn get(&self, filter: FilterEmployee) -> Result<Vec<EmployeeFlat>, AppError> {
        let mut query: QueryBuilder<'_, Postgres> = QueryBuilder::new(
            "SELECT
                e.id,
                e.name,
                e.last_name,
                e.middle_name,
                e.email,
                e.password,
                e.dismissed,
                e.created_at,
                e.updated_at,
                r.name AS role_name,
                r.created_at AS role_created_at,
                r.updated_at AS role_updated_at
            FROM employee AS e
            INNER JOIN role AS r ON e.role = r.name
            WHERE 1=1",
        );

        if let Some(id) = filter.id {
            query.push(" AND e.id = ").push_bind(id);
        }

        if let Some(name) = filter.name {
            query.push(" AND e.name = ").push_bind(name);
        }

        if let Some(last_name) = filter.last_name {
            query.push(" AND e.last_name = ").push_bind(last_name);
        }

        if let Some(middle_name) = filter.middle_name {
            query.push(" AND e.middle_name = ").push_bind(middle_name);
        }

        if let Some(email) = filter.email {
            query.push(" AND e.email = ").push_bind(email);
        }

        if let Some(role) = filter.role {
            query.push(" AND e.role = ").push_bind(role);
        }

        if let Some(dismissed) = filter.dismissed {
            query.push(" AND e.dismissed = ").push_bind(dismissed);
        }

        query
            .build_query_as::<EmployeeFlat>()
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::InternalServerError
            })
    }

    async fn get_by_email(&self, email: String) -> Result<EmployeeFlat, AppError> {
        query_as::<_, EmployeeFlat>(
            "SELECT
                e.id,
                e.name,
                e.last_name,
                e.middle_name,
                e.email,
                e.password,
                e.dismissed,
                e.created_at,
                e.updated_at,
                r.name AS role_name,
                r.created_at AS role_created_at,
                r.updated_at AS role_updated_at
            FROM employee AS e
            INNER JOIN role AS r ON e.role = r.name
            WHERE e.email = $1",
        )
        .bind(email)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| {
            error!("Database error: {e}");
            AppError::NotFound
        })
    }

    async fn dismiss(&self, email: String) -> Result<(), AppError> {
        let result = query("UPDATE employee SET dismissed = true, updated_at = NOW() WHERE dismissed = false AND email = $1")
            .bind(email)
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::BadRequest("Employee has already been dismissed".to_owned())
            })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
