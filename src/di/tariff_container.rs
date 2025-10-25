use std::sync::Arc;

use sqlx::PgPool;

use crate::adapters::tariff_adapter::TariffRepository;
use crate::application::tariff::TariffLogic;

pub struct TariffContainer {
    pub repo: Arc<TariffRepository>,
    pub logic: TariffLogic<TariffRepository>,
}

impl TariffContainer {
    pub fn new(postres: Arc<PgPool>) -> Self {
        let repo = Arc::new(TariffRepository::new(postres.clone()));
        let logic = TariffLogic::new(repo.clone());

        TariffContainer { repo, logic }
    }
}
