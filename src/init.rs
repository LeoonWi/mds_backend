use std::{error::Error, sync::Arc};

use mds_backend::{
    config::Config,
    db,
    features::{default_storage, role, tariff},
};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::build().expect("Failed to build config");
    let pool = db::connect(&config.db_url).expect("Failted to connect db");

    let roles = setup_roles(pool.clone()).await?;
    let role_default = roles.get_role("Специалист".to_string()).await?;

    let tariffs = setup_tariffs(pool.clone()).await?;
    let tariff_default = tariffs.get_tariff("Стандартный".to_string()).await?;

    let repo = Arc::new(default_storage::repository::new(pool.clone()));
    let settings = default_storage::logic::new(repo);
    settings
        .create_settings(role_default, tariff_default)
        .await?;

    Ok(())
}

async fn setup_roles(pool: Arc<PgPool>) -> Result<role::logic::Logic, Box<dyn Error>> {
    let repo = Arc::new(role::repository::new(pool));
    let roles = role::logic::new(repo);

    roles.create_role("Специалист").await?;
    roles.create_role("Менеджер").await?;
    roles.create_role("Суперадмин").await?;
    println!("Все роли успешно созданы!");

    return Ok(roles);
}

async fn setup_tariffs(pool: Arc<PgPool>) -> Result<tariff::logic::Logic, Box<dyn Error>> {
    let repo = Arc::new(tariff::repository::new(pool));
    let tariffs = tariff::logic::new(repo);

    tariffs.create_tariff("Стандартный").await?;
    tariffs.create_tariff("Бизнес").await?;
    println!("Все тарифы успешно созданы!");

    return Ok(tariffs);
}
