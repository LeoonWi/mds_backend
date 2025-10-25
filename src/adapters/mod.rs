pub mod tariff_adapter;

use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{error::Error, sync::Arc};

pub fn pg_connect(url: &String) -> Result<Arc<PgPool>, Box<dyn Error>> {
    // TODO Установить значение в 20 перед релизом (начальная точка 20-100)
    // Рассчитывать 2-4 * кол-во ядер CPU
    // Брать во внимание 1-2 подключения для клиента/воркера
    tracing::info!("Connection to postgres");

    match PgPoolOptions::new().max_connections(10).connect_lazy(&url) {
        Ok(v) => Ok(Arc::new(v)),
        Err(e) => {
            tracing::error!("Database error: {}", e.to_string());
            Err(e.to_string().into())
        }
    }
}
