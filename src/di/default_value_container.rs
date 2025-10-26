use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    adapters::default_value_adapter::DefaultValueRepository,
    application::default_value::DefaultValueLogic,
};

pub struct DefaultValueContainer {
    pub repo: Arc<DefaultValueRepository>,
    pub logic: DefaultValueLogic<DefaultValueRepository>,
}

impl DefaultValueContainer {
    pub fn new(postgres: Arc<PgPool>) -> Self {
        let repo = Arc::new(DefaultValueRepository::new(postgres));
        let logic = DefaultValueLogic::new(repo.clone());

        DefaultValueContainer { repo, logic }
    }
}
