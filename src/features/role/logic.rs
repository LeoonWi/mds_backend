use std::error::Error;
use std::sync::Arc;

use crate::features::role::repository::Repository;
use crate::models::role::Role;

pub struct Logic {
    repo: Arc<Repository>,
}

pub fn new(repo: Arc<Repository>) -> Logic {
    Logic { repo: repo }
}

impl Logic {
    pub async fn create_role(&self, name: &'static str) -> Result<(), Box<dyn Error>> {
        let span = tracing::info_span!("creating role: ", name);
        let _guard = span.enter();

        return self.repo.add(name).await.map_err(|e| e.into());
    }

    pub async fn get_roles(&self) -> Vec<Role> {
        self.repo.get().await
    }

    pub async fn get_role(&self, name: String) -> Result<Role, Box<dyn Error>> {
        let span = tracing::info_span!("getting role: ", name);
        let _guard = span.enter();

        return self.repo.get_by_name(name).await.map_err(|e| e.into());
    }
}
