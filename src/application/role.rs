use std::sync::Arc;

use crate::models::{error::AppError, role::Role};

pub trait RoleAdapter {
    fn save(&self, name: String) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
    fn get(&self) -> impl std::future::Future<Output = Result<Vec<Role>, AppError>> + Send;
    fn get_by_name(
        &self,
        name: String,
    ) -> impl std::future::Future<Output = Result<Role, AppError>> + Send;
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
