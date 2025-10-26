use std::sync::Arc;

use mds_backend::adapters;
use mds_backend::di::default_value_container;
use mds_backend::di::role_container;
use mds_backend::di::tariff_container;
use mds_backend::logger;

#[tokio::main]
async fn main() {
    logger::init_dev_logger();

    let config = mds_backend::config::Config::build().expect("Failed to build config");
    let postgres = adapters::pg_connect(&config.db_url, 1).expect("Failted to connect db");

    let tx = postgres
        .begin()
        .await
        .expect("Failed to create transaction");

    let tariff_container = Arc::new(tariff_container::TariffContainer::new(postgres.clone()));
    let role_container = Arc::new(role_container::RoleContainer::new(postgres.clone()));
    let default_value_container = Arc::new(default_value_container::DefaultValueContainer::new(
        postgres.clone(),
    ));

    // setup tariffs
    tariff_container
        .logic
        .create_tariff("Free".to_owned())
        .await
        .expect("Failed to create tariff");

    tariff_container
        .logic
        .create_tariff("Business".to_owned())
        .await
        .expect("Failed to create tariff");

    // setup roles
    role_container
        .logic
        .create_role("Employee".to_owned())
        .await
        .expect("Failed to create role");

    role_container
        .logic
        .create_role("Manager".to_owned())
        .await
        .expect("Failed to create role");

    role_container
        .logic
        .create_role("Superuser".to_owned())
        .await
        .expect("Failed to create role");

    // setup default values
    default_value_container
        .logic
        .set_default_values("Free".to_owned(), "Employee".to_owned())
        .await
        .expect("Failed to setup default values");

    tx.commit().await.expect("Failed to commit transaction");
    println!("Initialization of the system is completed");
}
