use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::role::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub password: String,
    pub role: Role,
    pub dismissed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(FromRow)]
pub struct EmployeeRow {
    // employee
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub password: String,
    pub dismissed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    // role
    pub role_name: String,
    pub role_created_at: DateTime<Utc>,
    pub role_updated_at: Option<DateTime<Utc>>,
}

impl From<EmployeeRow> for Employee {
    fn from(row: EmployeeRow) -> Self {
        Employee {
            id: row.id,
            name: row.name,
            last_name: row.last_name,
            middle_name: row.middle_name,
            email: row.email,
            password: row.password,
            role: Role {
                name: row.role_name,
                created_at: row.role_created_at,
                updated_at: row.role_updated_at,
            },
            dismissed: row.dismissed,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmployee {
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub password: String,
}
