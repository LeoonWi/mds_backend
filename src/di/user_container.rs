use std::sync::Arc;

use sqlx::PgPool;

use crate::adapters::default_value_adapter::DefaultValueRepository;
use crate::adapters::user::UserRepository;
use crate::application::user::UserLogic;

pub struct UserContainer {
    pub user_repo: Arc<UserRepository>,
    pub logic: UserLogic<UserRepository, DefaultValueRepository>,
}

impl UserContainer {
    pub fn new(postgres: Arc<PgPool>, default_value_repo: Arc<DefaultValueRepository>) -> Self {
        let user_repo = Arc::new(UserRepository::new(postgres));
        let logic = UserLogic::new(user_repo.clone(), default_value_repo);

        UserContainer {
            user_repo,
            logic,
        }
    }
}