use std::sync::Arc;

use crate::models::{error::AppError, service::Service};

pub trait ServiceAdapter {
    fn save(&self, name: String) -> impl Future<Output = Result<(), AppError>>;

    fn get(&self) -> impl Future<Output = Result<Vec<Service>, AppError>>;

    fn get_by_name(&self, name: String) -> impl Future<Output = Result<Service, AppError>>;

    fn delete(&self, name: String) -> impl Future<Output = Result<(), AppError>>;
}

pub struct ServiceLogic<R: ServiceAdapter> {
    repo: Arc<R>,
}

impl<R: ServiceAdapter> ServiceLogic<R> {
    pub fn new(repo: Arc<R>) -> Self {
        ServiceLogic { repo }
    }

    pub async fn create(&self, name: String) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::BadRequest("Empty name service".to_owned()));
        }

        if self.repo.get_by_name(name.clone()).await.is_ok() {
            return Err(AppError::Conflict);
        }

        self.repo.save(name).await
    }

    pub async fn get(&self) -> Result<Vec<Service>, AppError> {
        self.repo.get().await
    }

    pub async fn get_by_name(&self, name: String) -> Result<Service, AppError> {
        self.repo.get_by_name(name).await
    }
}
