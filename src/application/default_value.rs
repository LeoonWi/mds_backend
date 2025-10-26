use std::sync::Arc;

use crate::models::{error::AppError, role::Role, tariff::Tariff};

pub trait DefaultValueAdapter {
    fn save(
        &self,
        tariff: String,
        role: String,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
    fn save_tariff(
        &self,
        tariff: String,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
    fn save_role(
        &self,
        role: String,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
    fn get_tariff(&self) -> impl std::future::Future<Output = Result<Tariff, AppError>> + Send;
    fn get_role(&self) -> impl std::future::Future<Output = Result<Role, AppError>> + Send;
}

pub struct DefaultValueLogic<R>
where
    R: DefaultValueAdapter,
{
    repo: Arc<R>,
}

impl<R> DefaultValueLogic<R>
where
    R: DefaultValueAdapter,
{
    pub fn new(repo: Arc<R>) -> Self {
        DefaultValueLogic { repo }
    }

    pub async fn set_default_values(&self, tariff: String, role: String) -> Result<(), AppError> {
        self.repo.save(tariff, role).await
    }

    pub async fn set_tariff(&self, tariff: String) -> Result<(), AppError> {
        self.repo.save_tariff(tariff).await
    }

    pub async fn set_role(&self, role: String) -> Result<(), AppError> {
        self.repo.save_role(role).await
    }

    pub async fn get_tariff(&self) -> Result<Tariff, AppError> {
        self.repo.get_tariff().await
    }

    pub async fn get_role(&self) -> Result<Role, AppError> {
        self.repo.get_role().await
    }
}
