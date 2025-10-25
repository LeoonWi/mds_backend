use std::sync::Arc;

use crate::models::{error::AppError, role::Role};

pub trait RoleAdapter {
    async fn save(&self, name: String) -> Result<(), AppError>;
    async fn get(&self) -> Result<Vec<Role>, AppError>;
    async fn get_by_name(&self, name: String) -> Result<Role, AppError>;
}

pub struct RoleLogic<R>
where
    R: RoleAdapter,
{
    repo: Arc<R>,
}

impl<R> RoleLogic<R>
where
    R: RoleAdapter,
{
    pub fn new(repo: Arc<R>) -> Self {
        RoleLogic { repo }
    }

    pub async fn create_role(&self, name: String) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::BadRequest("Empty name role".to_owned()));
        }
        self.repo.save(name).await
    }

    pub async fn get_roles(&self) -> Vec<Role> {
        self.repo
            .get()
            .await
            .map_err(|_| Vec::<Role>::new())
            .unwrap()
    }

    pub async fn get_role(&self, name: String) -> Result<Role, AppError> {
        self.repo.get_by_name(name).await
    }
}
