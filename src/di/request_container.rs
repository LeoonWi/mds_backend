use std::sync::Arc;

use sqlx::PgPool;

use crate::adapters::request_adapter::RequestRepository;
use crate::application::request::RequestLogic;

pub struct RequestContainer {
    pub repo: Arc<RequestRepository>,
    pub request: RequestLogic<RequestRepository>,
}

impl RequestContainer {
    pub fn new(postgres: Arc<PgPool>) -> Self {
        let repo = Arc::new(RequestRepository::new(postgres));
        let request = RequestLogic::new(repo.clone());

        RequestContainer { repo, request }
    }
}
