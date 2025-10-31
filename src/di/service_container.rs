use std::sync::Arc;

use sqlx::PgPool;

use crate::adapters::service_adapter::ServiceRepository;
use crate::application::service::ServiceLogic;

pub struct ServiceContainer {
    pub repo: Arc<ServiceRepository>,
    pub service: ServiceLogic<ServiceRepository>,
}

impl ServiceContainer {
    pub fn new(postgres: Arc<PgPool>) -> Self {
        let repo = Arc::new(ServiceRepository::new(postgres));
        let service = ServiceLogic::new(repo.clone());

        ServiceContainer { repo, service }
    }
}
