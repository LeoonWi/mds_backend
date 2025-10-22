use std::error::Error;
use std::sync::Arc;

use crate::features::default_storage::repository::Repository;
use crate::models::default_storage::DefaultStorage;
use crate::models::role::Role;
use crate::models::tariff::Tariff;

pub struct Logic {
    repo: Arc<Repository>,
}

pub fn new(repo: Arc<Repository>) -> Logic {
    Logic { repo: repo }
}

impl Logic {
    pub async fn create_settings(&self, role: Role, tariff: Tariff) -> Result<(), Box<dyn Error>> {
        self.repo
            .add(role.name, tariff.name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get(&self) -> Result<DefaultStorage, Box<dyn Error>> {
        self.repo.get().await.map_err(|e| e.into())
    }
}
