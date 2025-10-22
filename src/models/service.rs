use chrono::{DateTime, Utc};

pub struct Service {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
