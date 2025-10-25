use std::sync::Arc;

use sqlx::PgPool;

use crate::adapters::role_adapter::RoleRepository;
use crate::application::role::RoleLogic;

pub struct RoleContainer {
    pub repo: Arc<RoleRepository>,
    pub logic: RoleLogic<RoleRepository>,
}

impl RoleContainer {
    pub fn new(postgres: Arc<PgPool>) -> Self {
        let repo = Arc::new(RoleRepository::new(postgres));
        let logic = RoleLogic::new(repo.clone());

        RoleContainer { repo, logic }
    }
}
