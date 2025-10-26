use std::sync::Arc;

use bcrypt::{DEFAULT_COST, hash};

use crate::application::default_value::DefaultValueAdapter;
use crate::models::user::{User, UserFlat};
use crate::models::error::AppError;
use crate::validate_email;

pub trait UserAdapter {
    fn save(
        &self,
        name: String,
        last_name: String,
        middle_name: Option<String>,
        email: String,
        phone: String,
        tariff: String,
        inn: Option<String>,
        snils: Option<String>,
        password: String,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
    
    fn get(&self) -> impl std::future::Future<Output = Result<Vec<UserFlat>, AppError>> + Send;
    
    fn get_by_email(
        &self,
        email: String,
    ) -> impl std::future::Future<Output = Result<UserFlat, AppError>> + Send;
    
    fn get_by_phone(
        &self,
        phone: String,
    ) -> impl std::future::Future<Output = Result<UserFlat, AppError>> + Send;
}

pub struct UserLogic<R, D>
where
    R: UserAdapter,
    D: DefaultValueAdapter,
{
    repo: Arc<R>,
    repo_default_value: Arc<D>,
}

impl<R, D> UserLogic<R, D>
where
    R: UserAdapter,
    D: DefaultValueAdapter,
{
    pub fn new(repo: Arc<R>, repo_default_value: Arc<D>) -> Self {
        UserLogic {
            repo,
            repo_default_value,
        }
    }

    pub async fn create_user(
        &self,
        name: String,
        last_name: String,
        middle_name: Option<String>,
        email: String,
        phone: String,
        password: String,
        inn: Option<String>,
        snils: Option<String>,
    ) -> Result<(), AppError> {
        if password.len() < 6 {
            return Err(AppError::BadRequest(
                "Password must be at least 6 characters".to_owned(),
            ));
        }

        let hash_password = hash(password, DEFAULT_COST).map_err(|_| {
            tracing::error!("Failed to hash password in application 'create_user'");
            AppError::BadRequest("Failed to hash password".to_owned())
        })?;

        let correct_email = validate_email::validate_email(email)?;

        let tariff = self
            .repo_default_value
            .get_tariff()
            .await
            .map_err(|_| {
                AppError::BadRequest("Need to set a basic tariff for creating a user".to_owned())
            })?
            .name;

        self.repo
            .save(
                name,
                last_name,
                middle_name,
                correct_email,
                phone,
                tariff,
                inn,
                snils,
                hash_password,
            )
            .await
    }

    pub async fn get_users(&self) -> Vec<User> {
        match self.repo.get().await {
            Ok(vec) => vec.into_iter().map(User::from).collect(),
            Err(_) => Vec::<User>::new(),
        }
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<User, AppError> {
        self.repo.get_by_email(email).await.map(User::from)
    }

    pub async fn get_user_by_phone(&self, phone: String) -> Result<User, AppError> {
        self.repo.get_by_phone(phone).await.map(User::from)
    }
}