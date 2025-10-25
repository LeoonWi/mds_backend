use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

use crate::{
    di::tariff_container::TariffContainer,
    models::{error::AppError, tariff::Tariff},
};

pub fn tariff_router(container: Arc<TariffContainer>) -> Router {
    Router::new()
        .route("/tariffs", get(get_tariffs))
        .route("/tariffs/{name}", get(get_tariff_by_name))
        .with_state(container)
}

async fn get_tariffs(State(container): State<Arc<TariffContainer>>) -> Json<Vec<Tariff>> {
    Json(container.logic.get_tariffs().await)
}

async fn get_tariff_by_name(
    State(container): State<Arc<TariffContainer>>,
    Path(name): Path<String>,
) -> Result<Json<Tariff>, AppError> {
    container.logic.get_tariff(name).await.map(|v| Json(v))
}
