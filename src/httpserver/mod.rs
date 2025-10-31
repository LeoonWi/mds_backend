mod client_handler;
mod employee_handler;
mod guard;
mod request_handler;
mod service_handler;

use std::sync::Arc;

use axum::extract::{MatchedPath, Request};
use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::di::client_container::ClientContainer;
use crate::di::employee_container::EmployeeContainer;
use crate::di::request_container::RequestContainer;
use crate::di::service_container::ServiceContainer;
use crate::httpserver::client_handler::client_router;
use crate::httpserver::employee_handler::employee_router;
use crate::httpserver::request_handler::request_router;
use crate::httpserver::service_handler::service_router;
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
            Self::Forbidden => (StatusCode::FORBIDDEN, ErrorResponse::new(None)),
        };

        (status, Json(error)).into_response()
    }
}

pub struct Server {
    ip: String,
    port: i16,
    service: Arc<ServiceContainer>,
    employee: Arc<EmployeeContainer>,
    client: Arc<ClientContainer>,
    request: Arc<RequestContainer>,
}

impl Server {
    pub fn new(
        ip: String,
        port: i16,
        service: Arc<ServiceContainer>,
        employee: Arc<EmployeeContainer>,
        client: Arc<ClientContainer>,
        request: Arc<RequestContainer>,
    ) -> Self {
        Server {
            ip,
            port,
            service,
            employee,
            client,
            request,
        }
    }

    pub async fn run(self) {
        // init logger
        logger::init_dev_logger();

        // init routers application
        let service_router = service_router(self.service);
        let employee_router = employee_router(self.employee);
        let client_router = client_router(self.client);
        let request_router = request_router(self.request);

        // init root router
        let app = Router::new()
            .merge(service_router)
            .merge(employee_router)
            .merge(client_router)
            .merge(request_router)
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
            )
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods([Method::POST, Method::GET, Method::PATCH, Method::DELETE])
                    .allow_headers(Any)
                    .allow_credentials(false),
            );

        // init server
        let addr = format!("{}:{}", self.ip, self.port);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        println!("Server running on {}", &addr);
        axum::serve(listener, app).await.unwrap();
    }
}
