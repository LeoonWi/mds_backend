use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Superuser = 3,
    Manager = 2,
    Employee = 1,
    Guest = 0,
}

#[derive(FromRow)]
pub struct EmployeeFlat {
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

impl From<EmployeeFlat> for Employee {
    fn from(value: EmployeeFlat) -> Self {
        Employee {
            id: value.id,
            name: value.name,
            last_name: value.last_name,
            middle_name: value.middle_name,
            email: value.email,
            role: value.role,
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
    pub role: Option<Role>,
}

#[derive(Deserialize)]
pub struct FilterEmployee {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub email: Option<String>,
    pub role: Option<Role>,
    pub dismissed: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginEmployee {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EmployeeWithService {
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub role: Role,
    pub dismissed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub services: Vec<String>,
}
