use std::sync::Arc;

use bcrypt::{DEFAULT_COST, hash, verify};

use crate::models::employee::{
    CreateEmployee, Employee, EmployeeFlat, EmployeeWithService, FilterEmployee, LoginEmployee,
    Role,
};
use crate::models::error::AppError;
use crate::validate_email;

pub trait EmployeeAdapter {
    fn save(
        &self,
        name: String,
        last_name: String,
        middle_name: Option<String>,
        email: String,
        role: Role,
        dismissed: bool,
        password: String,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    fn get(
        &self,
        filter: FilterEmployee,
    ) -> impl std::future::Future<Output = Result<Vec<EmployeeFlat>, AppError>> + Send;

    fn get_with_services(
        &self,
        filter: FilterEmployee,
    ) -> impl Future<Output = Result<Vec<EmployeeWithService>, AppError>>;

    fn get_by_email(
        &self,
        email: String,
    ) -> impl std::future::Future<Output = Result<EmployeeFlat, AppError>> + Send;

    fn change_role(&self, email: String, role: Role) -> impl Future<Output = Result<(), AppError>>;

    fn add_service(&self, id: i64, service: String) -> impl Future<Output = Result<(), AppError>>;

    fn remove_service(
        &self,
        id: i64,
        service: String,
    ) -> impl Future<Output = Result<(), AppError>>;

    fn dismiss(
        &self,
        email: String,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
}

pub struct EmployeeLogic<R: EmployeeAdapter> {
    repo: Arc<R>,
}

impl<R> EmployeeLogic<R>
where
    R: EmployeeAdapter,
{
    pub fn new(repo: Arc<R>) -> Self {
        EmployeeLogic { repo }
    }

    pub async fn create_employee(&self, payload: CreateEmployee) -> Result<(), AppError> {
        if payload.password.len() < 6 {
            return Err(AppError::BadRequest(
                "Password must be at least 6 characters".to_owned(),
            ));
        }

        let hash_password = hash(payload.password, DEFAULT_COST).map_err(|_| {
            tracing::error!("Failed to hash password in application 'create_employee'");
            AppError::BadRequest("Failed to hash password".to_owned())
        })?;

        let correct_email = validate_email::validate_email(payload.email)?;

        let role = match payload.role {
            Some(role) => role,
            None => Role::Employee,
        };

        self.repo
            .save(
                payload.name,
                payload.last_name,
                payload.middle_name,
                correct_email,
                role,
                false,
                hash_password,
            )
            .await
    }

    pub async fn login(&self, payload: LoginEmployee) -> Result<(i64, Role), AppError> {
        let employee = self.repo.get_by_email(payload.email).await?;

        if !verify(payload.password, &employee.password).unwrap() {
            return Err(AppError::BadRequest("Wrong user password".to_owned()));
        }

        Ok((employee.id, employee.role))
    }

    pub async fn get_employees(&self, filter: FilterEmployee) -> Vec<Employee> {
        match self.repo.get(filter).await {
            Ok(vec) => vec.into_iter().map(Employee::from).collect(),
            Err(_) => Vec::<Employee>::new(),
        }
    }

    pub async fn get_employee_by_email(&self, email: String) -> Result<Employee, AppError> {
        self.repo.get_by_email(email).await.map(Employee::from)
    }

    pub async fn change_role(
        &self,
        self_role: Role,
        email: String,
        role: Role,
    ) -> Result<(), AppError> {
        let expected_change_role = self.repo.get_by_email(email.clone()).await?;
        if self_role < expected_change_role.role {
            return Err(AppError::BadRequest(
                "Cannot delete an employee above your own position".to_owned(),
            ));
        }

        self.repo.change_role(email, role).await
    }

    pub async fn dismiss_employee(&self, self_role: Role, email: String) -> Result<(), AppError> {
        let expected_remove = self.repo.get_by_email(email.clone()).await?;
        if self_role < expected_remove.role {
            return Err(AppError::BadRequest(
                "Cannot delete an employee above your own position".to_owned(),
            ));
        }

        self.repo.dismiss(email).await
    }

    pub async fn add_service(&self, id: i64, service: String) -> Result<(), AppError> {
        if service.is_empty() {
            return Err(AppError::BadRequest(
                "Length service cannot be zero".to_owned(),
            ));
        }
        self.repo.add_service(id, service).await
    }

    pub async fn remove_service(&self, id: i64, service: String) -> Result<(), AppError> {
        self.repo.remove_service(id, service).await
    }

    pub async fn get_with_services(&self, filter: FilterEmployee) -> Vec<EmployeeWithService> {
        self.repo
            .get_with_services(filter)
            .await
            .map_err(|_| Vec::<EmployeeWithService>::new())
            .unwrap()
    }
}
