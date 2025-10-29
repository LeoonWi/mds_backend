pub mod adapters;
mod application;
pub mod config;
pub mod di;
mod httpserver;
mod logger;
pub mod models;
mod phoneparse;
mod validate_email;

use std::{error::Error, sync::Arc};

use crate::di::{
    client_container, default_value_container, employee_container, role_container, tariff_container,
};

pub async fn run() -> Result<(), Box<dyn Error>> {
    let config = config::Config::build()?;
    let postgres = adapters::pg_connect(&config.db_url, 10)?;

    // di containers
    let tariff_container = Arc::new(tariff_container::TariffContainer::new(postgres.clone()));

    let role_container = Arc::new(role_container::RoleContainer::new(postgres.clone()));

    let default_value_container = Arc::new(default_value_container::DefaultValueContainer::new(
        postgres.clone(),
    ));

    let service_container = Arc::new(di::service_container::ServiceContainer::new(
        postgres.clone(),
    ));

    let employee_container = Arc::new(employee_container::EmployeeContainer::new(
        postgres.clone(),
        default_value_container.repo.clone(),
    ));

    let client_container = Arc::new(client_container::ClientContainer::new(
        postgres.clone(),
        default_value_container.repo.clone(),
    ));

    let request_container = Arc::new(di::request_container::RequestContainer::new(
        postgres.clone(),
    ));

    // http server
    let server = httpserver::Server::new(
        config.ip,
        config.port,
        tariff_container,
        role_container,
        default_value_container,
        service_container,
        employee_container,
        client_container,
        request_container,
    );
    server.run().await;

    Ok(())
}
