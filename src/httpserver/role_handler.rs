use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

use crate::models::error::AppError;
use crate::{di::role_container::RoleContainer, models::role::Role};

pub fn role_router(container: Arc<RoleContainer>) -> Router {
    Router::new()
        .route("/", get(get_roles))
        .route("/{name}", get(get_role_by_name))
        .with_state(container)
}

async fn get_roles(State(container): State<Arc<RoleContainer>>) -> Json<Vec<Role>> {
    Json(container.logic.get_roles().await)
}

async fn get_role_by_name(
    State(container): State<Arc<RoleContainer>>,
    Path(name): Path<String>,
) -> Result<Json<Role>, AppError> {
    container.logic.get_role(name).await.map(|v| Json(v))
}
