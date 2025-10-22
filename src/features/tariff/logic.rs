use std::error::Error;
use std::sync::Arc;

use crate::features::tariff::repository::Repository;
use crate::models::tariff::Tariff;

pub struct Logic {
    repo: Arc<Repository>,
}

pub fn new(repo: Arc<Repository>) -> Logic {
    Logic { repo: repo }
}

impl Logic {
    pub async fn create_tariff(&self, name: &'static str) -> Result<(), Box<dyn Error>> {
        let span = tracing::info_span!("creating tariff: ", name);
        let _guard = span.enter();

        return self.repo.add(name).await.map_err(|e| e.into());
    }

    pub async fn get_roles(&self) -> Vec<Tariff> {
        self.repo.get().await
    }

    pub async fn get_tariff(&self, name: String) -> Result<Tariff, Box<dyn Error>> {
        let span = tracing::info_span!("getting tariff: ", name);
        let _guard = span.enter();

        return self.repo.get_by_name(name).await.map_err(|e| e.into());
    }
}
