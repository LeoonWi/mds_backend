use std::sync::Arc;

use chrono::{DateTime, Utc};

use crate::models::error::AppError;
use crate::models::request::{
    CreateRequest, FilterRequest, Priority, Request, RequestFlat, Status,
};

pub trait RequestAdapter {
    fn save(&self, payload: CreateRequest) -> impl Future<Output = Result<(), AppError>>;

    fn get(
        &self,
        filter: FilterRequest,
    ) -> impl Future<Output = Result<Vec<RequestFlat>, AppError>>;

    fn set_status(
        &self,
        id: i64,
        status: Status,
        closed_at: Option<DateTime<Utc>>,
    ) -> impl Future<Output = Result<(), AppError>>;

    fn set_priority(
        &self,
        id: i64,
        priority: Priority,
    ) -> impl Future<Output = Result<(), AppError>>;

    fn set_employee(&self, id: i64, employee_id: i64)
    -> impl Future<Output = Result<(), AppError>>;

    fn delete(&self, id: i64) -> impl Future<Output = Result<(), AppError>>;
}

pub struct RequestLogic<R: RequestAdapter> {
    repo: Arc<R>,
}

impl<R: RequestAdapter> RequestLogic<R> {
    pub fn new(repo: Arc<R>) -> Self {
        RequestLogic { repo }
    }

    pub async fn create(&self, payload: CreateRequest) -> Result<(), AppError> {
        if payload.name.is_empty() {
            return Err(AppError::BadRequest(
                "Name request cannot be empty".to_owned(),
            ));
        }

        if payload.desc.is_empty() {
            return Err(AppError::BadRequest(
                "Description cannot be empty".to_owned(),
            ));
        }

        self.repo.save(payload).await
    }

    pub async fn get(&self, filter: FilterRequest) -> Vec<Request> {
        match self.repo.get(filter).await {
            Ok(vec) => vec.into_iter().map(Request::from).collect(),
            Err(_) => Vec::<Request>::new(),
        }
    }

    pub async fn set_status(&self, id: i64, new_status: Status) -> Result<(), AppError> {
        let closed_at = match new_status {
            Status::Approved => Some(Utc::now()),
            _ => None,
        };

        self.repo.set_status(id, new_status, closed_at).await
    }

    pub async fn set_priority(&self, id: i64, new_priority: Priority) -> Result<(), AppError> {
        self.repo.set_priority(id, new_priority).await
    }

    pub async fn change_employee(&self, id: i64, employee_id: i64) -> Result<(), AppError> {
        self.repo.set_employee(id, employee_id).await
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        self.repo.delete(id).await
    }
}
