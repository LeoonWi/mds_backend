use std::sync::Arc;

use sqlx::PgPool;

use crate::adapters::client_adapter::ClientRepository;
use crate::application::client::ClientLogic;

pub struct ClientContainer {
    pub repo: Arc<ClientRepository>,
    pub logic: ClientLogic<ClientRepository>,
}

impl ClientContainer {
    pub fn new(postgres: Arc<PgPool>) -> Self {
        let repo = Arc::new(ClientRepository::new(postgres));
        let logic = ClientLogic::new(repo.clone());

        ClientContainer { repo, logic }
    }
}
