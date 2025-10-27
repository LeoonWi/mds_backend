use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::role::Role;

#[derive(Serialize, Deserialize)]
pub struct Employee {
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub role: Role,
    pub dismissed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(FromRow)]
pub struct EmployeeFlat {
    // employee data
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub password: String,
    pub dismissed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    // role data
    pub role_name: String,
    pub role_created_at: DateTime<Utc>,
    pub role_updated_at: Option<DateTime<Utc>>,
}

impl From<EmployeeFlat> for Employee {
    fn from(value: EmployeeFlat) -> Self {
        Employee {
            id: value.id,
            name: value.name,
            last_name: value.last_name,
            middle_name: value.middle_name,
            email: value.email,
            role: Role {
                name: value.role_name,
                created_at: value.role_created_at,
                updated_at: value.role_updated_at,
            },
            dismissed: value.dismissed,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateEmployee {
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Deserialize)]
pub struct FilterEmployee {
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub dismissed: Option<bool>,
}
