use std::sync::Arc;

use axum::{Json, extract::State};

use crate::features::tariff::logic::Logic;
use crate::models::tariff::Tariff;

pub async fn get_tariffs(State(logic): State<Arc<Logic>>) -> Json<Vec<Tariff>> {
    let span = tracing::info_span!("getting tarrifs");
    let _guard = span.enter();

    let result = logic.get_roles().await;
    return Json(result);
}
