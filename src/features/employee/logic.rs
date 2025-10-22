use std::sync::Arc;

use crate::{features::employee::repository::Repository, models::default_storage::DefaultStorage};

pub struct Logic {
    repo: Arc<Repository>,
    default_storage: Arc<DefaultStorage>,
}

pub fn new(repo: Arc<Repository>, default_storage: Arc<DefaultStorage>) -> Logic {
    Logic {
        repo: repo,
        default_storage: default_storage,
    }
}
