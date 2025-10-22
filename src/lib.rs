pub mod config;
pub mod db;
pub mod features;
mod logger;
pub mod models;

use axum::Router;

use crate::config::Config;
use crate::features::{role, tariff};
use std::error::Error;

pub async fn server_run() -> Result<(), Box<dyn Error>> {
    let config = Config::build()?;

    match config.profile.as_str() {
        "prod" => logger::init_prod_logger(),
        _ => logger::init_dev_logger(),
    }

    let pool = db::connect(&config.db_url)?;

    let role = role::new(pool.clone());
    let tariff = tariff::new(pool.clone());

    let app = Router::new().merge(role).merge(tariff);

    tracing::info!("Server running on {}:{}", config.ip, config.port);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.ip, config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
