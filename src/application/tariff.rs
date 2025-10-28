use std::sync::Arc;

use crate::models::{error::AppError, tariff::Tariff};

pub trait TariffAdapter {
    fn save(&self, name: String) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
    fn get(&self) -> impl std::future::Future<Output = Result<Vec<Tariff>, AppError>> + Send;
    fn get_by_name(
        &self,
        name: String,
    ) -> impl std::future::Future<Output = Result<Tariff, AppError>> + Send;
}

pub struct TariffLogic<R>
where
    R: TariffAdapter,
{
    repo: Arc<R>,
}

impl<R> TariffLogic<R>
where
    R: TariffAdapter,
{
    pub fn new(repo: Arc<R>) -> Self {
        TariffLogic { repo }
    }

    pub async fn create_tariff(&self, name: String) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::BadRequest("Empty name tariff".to_owned()));
        }

        if self.repo.get_by_name(name.clone()).await.is_ok() {
            return Err(AppError::Conflict);
        }

        self.repo.save(name).await
    }

    pub async fn get_tariffs(&self) -> Vec<Tariff> {
        self.repo
            .get()
            .await
            .map_err(|_| Vec::<Tariff>::new())
            .unwrap()
    }

    pub async fn get_tariff(&self, name: String) -> Result<Tariff, AppError> {
        self.repo.get_by_name(name).await
    }
}
