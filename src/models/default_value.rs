use sqlx::FromRow;

#[derive(FromRow)]
pub struct DefaultValue {
    pub tariff: String,
    pub role: String,
}
