mod adapters;
mod application;
mod config;
mod di;
mod httpserver;
mod logger;
mod models;

use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config::Config::build()?;
    let postgres = adapters::pg_connect(&config.db_url)?;
    let tariff_container = Arc::new(di::tariff_container::TariffContainer::new(postgres.clone()));
    let role_container = Arc::new(di::role_container::RoleContainer::new(postgres.clone()));
    let server = httpserver::Server::new(config.port, tariff_container, role_container);
    server.run().await;

    Ok(())
}
