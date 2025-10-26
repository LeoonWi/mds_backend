use std::sync::Arc;

use bcrypt::{DEFAULT_COST, hash};

use crate::application::default_value::DefaultValueAdapter;
use crate::models::employee::{Employee, EmployeeFlat};
use crate::models::error::AppError;
use crate::validate_email;

pub trait EmployeeAdapter {
    fn save(
        &self,
        name: String,
        last_name: String,
        middle_name: Option<String>,
        email: String,
        role: String,
        dismissed: bool,
        password: String,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
    fn get(&self) -> impl std::future::Future<Output = Result<Vec<EmployeeFlat>, AppError>> + Send;
    fn get_by_email(
        &self,
        email: String,
    ) -> impl std::future::Future<Output = Result<EmployeeFlat, AppError>> + Send;
    fn dismiss(
        &self,
        email: String,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
}

pub struct EmployeeLogic<R, D>
where
    R: EmployeeAdapter,
    D: DefaultValueAdapter,
{
    repo: Arc<R>,
    repo_default_value: Arc<D>,
}

impl<R, D> EmployeeLogic<R, D>
where
    R: EmployeeAdapter,
    D: DefaultValueAdapter,
{
    pub fn new(repo: Arc<R>, repo_default_value: Arc<D>) -> Self {
        EmployeeLogic {
            repo,
            repo_default_value,
        }
    }

    pub async fn create_employee(
        &self,
        name: String,
        last_name: String,
        middle_name: Option<String>,
        email: String,
        password: String,
    ) -> Result<(), AppError> {
        if password.len() < 5 {
            return Err(AppError::BadRequest(
                "Password must be at least 6 characters".to_owned(),
            ));
        }

        let hash_password = hash(password, DEFAULT_COST).map_err(|_| {
            tracing::error!("Failed to hash password in application 'create_employee'");
            AppError::BadRequest("Failed to hash password".to_owned())
        })?;

        let correct_email = validate_email::validate_email(email)?;

        let role = self
            .repo_default_value
            .get_role()
            .await
            .map_err(|_| {
                AppError::BadRequest("Need to set a basic role for creating an employee".to_owned())
            })?
            .name;

        self.repo
            .save(
                name,
                last_name,
                middle_name,
                correct_email,
                role,
                false,
                hash_password,
            )
            .await
    }

    pub async fn get_employees(&self) -> Vec<Employee> {
        match self.repo.get().await {
            Ok(vec) => vec.into_iter().map(Employee::from).collect(),
            Err(_) => Vec::<Employee>::new(),
        }
    }

    pub async fn get_employee_by_email(&self, email: String) -> Result<Employee, AppError> {
        self.repo.get_by_email(email).await.map(Employee::from)
    }

    pub async fn dismiss_employee(&self, email: String) -> Result<(), AppError> {
        self.repo.dismiss(email).await
    }
}
