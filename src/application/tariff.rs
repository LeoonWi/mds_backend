use std::sync::Arc;

use crate::models::{error::AppError, tariff::Tariff};

pub trait TariffAdapter {
    async fn save(&self, name: String) -> Result<(), AppError>;
    async fn get(&self) -> Result<Vec<Tariff>, AppError>;
    async fn get_by_name(&self, name: String) -> Result<Tariff, AppError>;
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
