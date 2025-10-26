pub mod adapters;
mod application;
pub mod config;
pub mod di;
mod httpserver;
pub mod logger;
mod models;
mod validate_email;

use std::{error::Error, sync::Arc};

use crate::di::{default_value_container, employee_container, role_container, tariff_container};

pub async fn run() -> Result<(), Box<dyn Error>> {
    let config = config::Config::build()?;
    let postgres = adapters::pg_connect(&config.db_url, 10)?;

    let tariff_container = Arc::new(tariff_container::TariffContainer::new(postgres.clone()));
    let role_container = Arc::new(role_container::RoleContainer::new(postgres.clone()));
    let default_value_container = Arc::new(default_value_container::DefaultValueContainer::new(
        postgres.clone(),
    ));
    let employee_container = Arc::new(employee_container::EmployeeContainer::new(
        postgres.clone(),
        default_value_container.repo.clone(),
    ));

    let server = httpserver::Server::new(
        config.ip,
        config.port,
        tariff_container,
        role_container,
        default_value_container,
        employee_container,
    );
    server.run().await;

    Ok(())
}
