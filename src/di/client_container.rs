use std::sync::Arc;

use sqlx::PgPool;

use crate::adapters::client_adapter::ClientRepository;
use crate::adapters::default_value_adapter::DefaultValueRepository;
use crate::application::client::ClientLogic;

pub struct ClientContainer {
    pub repo: Arc<ClientRepository>,
    pub logic: ClientLogic<ClientRepository, DefaultValueRepository>,
}

impl ClientContainer {
    pub fn new(postgres: Arc<PgPool>, default_value_repo: Arc<DefaultValueRepository>) -> Self {
        let repo = Arc::new(ClientRepository::new(postgres));
        let logic = ClientLogic::new(repo.clone(), default_value_repo);

        ClientContainer { repo, logic }
    }
}
