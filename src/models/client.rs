use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "tariff", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Tariff {
    Free,
    Business,
}

#[derive(Debug, FromRow)]
pub struct ClientFlat {
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub tariff: Tariff,
    pub inn: Option<String>,
    pub snils: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
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
        Client {
            id: value.id,
            name: value.name,
            last_name: value.last_name,
            middle_name: value.middle_name,
            email: value.email,
            phone: value.phone,
            tariff: value.tariff,
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
    pub id: Option<i64>,
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub tariff: Option<Tariff>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginClient {
    pub email: String,
    pub password: String,
}
