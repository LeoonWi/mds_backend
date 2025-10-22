use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DefaultStorage {
    pub id: i32,
    pub tariff: String,
    pub role: String,
}
