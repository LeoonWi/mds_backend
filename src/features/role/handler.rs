use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use tracing::info_span;

use crate::features::role::logic::Logic;
use crate::models::role::Role;

pub async fn get_roles(State(logic): State<Arc<Logic>>) -> Json<Vec<Role>> {
    let span = info_span!("getting roles");
    let _guard = span.enter();

    let result = logic.get_roles().await;
    return Json(result);
}
