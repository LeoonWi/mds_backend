use std::sync::Arc;

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, QueryBuilder, query};
use tracing::error;

use crate::application::request::RequestAdapter;
use crate::models::error::AppError;
use crate::models::request::{CreateRequest, FilterRequest, Priority, RequestFlat, Status};

pub struct RequestRepository {
    pub pool: Arc<PgPool>,
}

impl RequestRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        RequestRepository { pool }
    }
}

impl RequestAdapter for RequestRepository {
    async fn save(&self, payload: CreateRequest) -> Result<(), AppError> {
        query(
            "INSERT INTO request (name, service, owner_id, employee_id, priority, \"desc\", status, desired_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(payload.name)
        .bind(payload.service)
        .bind(payload.owner_id)
        .bind(payload.employee_id)
        .bind(payload.priority)
        .bind(payload.desc)
        .bind(Status::New)
        .bind(payload.desired_at)
        .execute(&*self.pool)
        .await
        .map_err(|e| {
            error!("Database error: {e}");
            AppError::BadRequest(e.to_string())
        })?;

        Ok(())
    }

    async fn get(&self, filter: FilterRequest) -> Result<Vec<RequestFlat>, AppError> {
        let mut query = QueryBuilder::<'_, Postgres>::new(
            "SELECT
                -- request
                r.id,
                r.name,
                r.priority,
                r.desc,
                r.status,
                r.created_at,
                r.updated_at,
                r.desired_at,
                r.closed_at,
                
                -- service (can be NULL)
                s.name AS service_name,
                s.created_at AS service_created_at,
                s.updated_at AS service_updated_at,

                -- owner (user)
                c.id AS owner_id,
                c.name AS owner_name,
                c.last_name AS owner_last_name,
                c.middle_name AS owner_middle_name,
                c.email AS owner_email,
                c.phone AS owner_phone,
                c.tariff AS owner_tariff,
                c.password AS owner_password,
                c.inn AS owner_inn,
                c.snils AS owner_snils,
                c.created_at AS owner_created_at,
                c.updated_at AS owner_updated_at,

                -- employee
                e.id AS employee_id,
                e.name AS employee_name,
                e.last_name AS employee_last_name,
                e.middle_name AS employee_middle_name,
                e.email AS employee_email,
                e.password AS employee_password,
                e.role AS employee_role,
                e.dismissed AS employee_dismissed,
                e.created_at AS employee_created_at,
                e.updated_at AS employee_updated_at

                FROM request AS r
                LEFT JOIN service AS s ON s.name = r.service
                INNER JOIN \"user\" AS c ON c.id = r.owner_id
                LEFT JOIN employee AS e ON e.id = r.employee_id
                WHERE 1=1
                ",
        );

        if let Some(id) = filter.id {
            query.push(" AND r.id = ").push_bind(id);
        }

        if let Some(name) = filter.name {
            query.push(" AND r.name = ").push_bind(name);
        }

        if let Some(service) = filter.service {
            query.push(" AND r.service = ").push_bind(service);
        }

        if let Some(owner_id) = filter.owner_id {
            query.push(" AND r.owner_id = ").push_bind(owner_id);
        }

        if let Some(employee_id) = filter.employee_id {
            query.push(" AND r.employee_id = ").push_bind(employee_id);
        }

        if let Some(priority) = filter.priority {
            query.push(" AND r.priority = ").push_bind(priority);
        }

        if let Some(status) = filter.status {
            query.push(" AND r.status = ").push_bind(status);
        }

        if let Some(desired_at) = filter.desired_at {
            query.push(" AND r.desired_at = ").push_bind(desired_at);
        }

        query.push(" ORDER BY r.created_at DESC");

        query
            .build_query_as::<RequestFlat>()
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::InternalServerError
            })
    }

    async fn set_status(
        &self,
        id: i64,
        status: Status,
        closed_at: Option<DateTime<Utc>>,
    ) -> Result<(), AppError> {
        let row = query(
            "UPDATE request
            SET status = $1, updated_at = NOW(), closed_at = $2
            WHERE id = $3",
        )
        .bind(status)
        .bind(closed_at)
        .bind(id)
        .execute(&*self.pool)
        .await
        .map_err(|e| {
            error!("Database error: {e}");
            AppError::InternalServerError
        })?;

        if row.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }

    async fn set_priority(&self, id: i64, priority: Priority) -> Result<(), AppError> {
        let row = query(
            "UPDATE request
            SET priority = $1, updated_at = NOW()
            WHERE id = $2",
        )
        .bind(priority)
        .bind(id)
        .execute(&*self.pool)
        .await
        .map_err(|e| {
            error!("Database error: {e}");
            AppError::InternalServerError
        })?;

        if row.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }

    async fn set_employee(&self, id: i64, employee_id: i64) -> Result<(), AppError> {
        let row = query(
            "UPDATE request
            SET employee_id = $1, updated_at = NOW()
            WHERE id = $2",
        )
        .bind(employee_id)
        .bind(id)
        .execute(&*self.pool)
        .await
        .map_err(|e| {
            error!("Database error: {e}");
            AppError::InternalServerError
        })?;

        if row.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }

    async fn delete(&self, id: i64) -> Result<(), AppError> {
        let row = query("DELETE FROM request WHERE id = $1")
            .bind(id)
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                error!("Database error: {e}");
                AppError::InternalServerError
            })?;

        if row.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
