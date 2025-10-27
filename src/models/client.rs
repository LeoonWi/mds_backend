use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::tariff::Tariff;

#[derive(FromRow)]
pub struct ClientFlat {
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub inn: Option<String>,
    pub snils: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,

    // tariff data
    pub tariff_name: String,
    pub tariff_created_at: DateTime<Utc>,
    pub tariff_updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct Client {
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub phone: String,
    pub tariff: Tariff,
    pub inn: Option<String>,
    pub snils: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<ClientFlat> for Client {
    fn from(value: ClientFlat) -> Self {
        Self {
            id: value.id,
            name: value.name,
            last_name: value.last_name,
            middle_name: value.middle_name,
            email: value.email,
            phone: value.phone,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClient {
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub inn: Option<String>,
    pub snils: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterClient {
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub tariff: Option<String>,
}
