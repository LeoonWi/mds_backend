mod client_handler;
mod employee_handler;
mod role_handler;
mod service_handler;
mod tariff_handler;

use std::sync::Arc;

use axum::extract::{MatchedPath, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;

use crate::di::client_container::ClientContainer;
use crate::di::default_value_container::DefaultValueContainer;
use crate::di::employee_container::EmployeeContainer;
use crate::di::role_container::RoleContainer;
use crate::di::service_container::ServiceContainer;
use crate::di::tariff_container::TariffContainer;
use crate::httpserver::client_handler::client_router;
use crate::httpserver::employee_handler::employee_router;
use crate::httpserver::role_handler::role_router;
use crate::httpserver::service_handler::service_router;
use crate::httpserver::tariff_handler::tariff_router;
use crate::logger;
use crate::models::error::AppError;

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    timestamp: DateTime<Utc>,
    message: Option<String>,
}

impl ErrorResponse {
    fn new(message: Option<String>) -> Self {
        ErrorResponse {
            message,
            timestamp: Utc::now(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            Self::Conflict => (StatusCode::CONFLICT, ErrorResponse::new(None)),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, ErrorResponse::new(Some(msg))),
            Self::NotFound => (StatusCode::NOT_FOUND, ErrorResponse::new(None)),
            Self::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse::new(None))
            }
        };

        (status, Json(error)).into_response()
    }
}

pub struct Server {
    ip: String,
    port: i16,
    tariff: Arc<TariffContainer>,
    role: Arc<RoleContainer>,
    default_value: Arc<DefaultValueContainer>,
    service: Arc<ServiceContainer>,
    employee: Arc<EmployeeContainer>,
    client: Arc<ClientContainer>,
}

impl Server {
    pub fn new(
        ip: String,
        port: i16,
        tariff: Arc<TariffContainer>,
        role: Arc<RoleContainer>,
        default_value: Arc<DefaultValueContainer>,
        service: Arc<ServiceContainer>,
        employee: Arc<EmployeeContainer>,
        client: Arc<ClientContainer>,
    ) -> Self {
        Server {
            ip,
            port,
            tariff,
            role,
            default_value,
            service,
            employee,
            client,
        }
    }

    pub async fn run(self) {
        // init logger
        logger::init_dev_logger();

        // init routers application
        let tariff_router = tariff_router(self.tariff);
        let role_router = role_router(self.role);
        let service_router = service_router(self.service);
        let employee_router = employee_router(self.employee);
        let client_router = client_router(self.client);

        // init root router
        let app = Router::new()
            .merge(tariff_router)
            .merge(role_router)
            .merge(service_router)
            .merge(employee_router)
            .merge(client_router)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|req: &Request| {
                        let method = req.method();
                        let uri = req.uri();
                        let matched_path = req
                            .extensions()
                            .get::<MatchedPath>()
                            .map(|matched_path| matched_path.as_str());

                        tracing::debug_span!("request ", %method, %uri, matched_path)
                    })
                    .on_failure(()),
            );

        // init server
        let addr = format!("{}:{}", self.ip, self.port);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        println!("Server running on {}", &addr);
        axum::serve(listener, app).await.unwrap();
    }
}
