use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::tariff::Tariff;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub tariff: Tariff,
    pub inn: Option<String>,
    pub snils: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(FromRow)]
pub struct UserFlat {
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub tariff: String,
    pub inn: Option<String>,
    pub snils: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub tariff_name: String,
    pub tariff_created_at: DateTime<Utc>,
    pub tariff_updated_at: Option<DateTime<Utc>>,
}

impl From<UserFlat> for User {
    fn from(value: UserFlat) -> Self {
        User {
            id: value.id,
            name: value.name,
            last_name: value.last_name,
            middle_name: value.middle_name,
            email: value.email,
            phone: value.phone,
            password: value.password,
            tariff: Tariff {
                name: value.tariff_name,
                created_at: value.tariff_created_at,
                updated_at: value.tariff_updated_at,
            },
            inn: value.inn,
            snils: value.snils,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}