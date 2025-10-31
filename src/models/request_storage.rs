use crate::models::employee::Role;

#[derive(Clone)]
pub struct RequestStorage {
    pub user_id: i64,
    pub role: Role,
}
