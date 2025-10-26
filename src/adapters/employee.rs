use std::sync::Arc;

use sqlx::{PgPool, query, query_as};
use tracing::error;

use crate::application::employee::EmployeeAdapter;
use crate::models::employee::EmployeeFlat;
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
            "INSERT INTO employee
            (name, last_name, middle_name, email, password, role, dismissed)
            VALUES
            ($1, $2, $3, $4, $5, $6, $7)",
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

    async fn get(&self) -> Result<Vec<EmployeeFlat>, AppError> {
        query_as::<_, EmployeeFlat>("SELECT * FROM employee")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::InternalServerError
            })
    }

    async fn get_by_email(&self, email: String) -> Result<EmployeeFlat, AppError> {
        query_as::<_, EmployeeFlat>("SELECT * FROM employee WHERE email = $1")
            .bind(email)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::NotFound
            })
    }

    async fn dismiss(&self, email: String) -> Result<(), AppError> {
        query("UPDATE employee SET dismiss = true WHERE dismiss = false AND email = $1")
            .bind(email)
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::BadRequest("Employee has already been dismissed".to_owned())
            })?;

        Ok(())
    }
}
