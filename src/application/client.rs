use std::sync::Arc;

use bcrypt::DEFAULT_COST;
use phonenumber::country::Id::RU;

use crate::application::default_value::DefaultValueAdapter;
use crate::models::client::{Client, ClientFlat, CreateClient, FilterClient};
use crate::models::error::AppError;
use crate::phoneparse::phoneparse;
use crate::validate_email::validate_email;

fn only_digits(text: String) -> String {
    text.chars().filter(|c| c.is_ascii_digit()).collect()
}

pub trait ClientAdapter {
    fn save(
        &self,
        name: String,
        last_name: String,
        middle_name: Option<String>,
        email: String,
        phone: String,
        password: String,
        tariff: String,
        inn: Option<String>,
        snils: Option<String>,
    ) -> impl Future<Output = Result<(), AppError>>;

    fn get(&self, filter: FilterClient) -> impl Future<Output = Result<Vec<ClientFlat>, AppError>>;

    fn get_by_email(&self, email: String) -> impl Future<Output = Result<ClientFlat, AppError>>;

    fn delete(&self, email: String) -> impl Future<Output = Result<(), AppError>>;
}

pub struct ClientLogic<R, D>
where
    R: ClientAdapter,
    D: DefaultValueAdapter,
{
    repo: Arc<R>,
    repo_default_value: Arc<D>,
}

impl<R, D> ClientLogic<R, D>
where
    R: ClientAdapter,
    D: DefaultValueAdapter,
{
    pub fn new(repo: Arc<R>, repo_default_value: Arc<D>) -> Self {
        ClientLogic {
            repo,
            repo_default_value,
        }
    }

    pub async fn create_client(&self, payload: CreateClient) -> Result<(), AppError> {
        // Валидация полей
        if payload.name.is_empty() || payload.last_name.is_empty() {
            return Err(AppError::BadRequest("Empty fullname user".to_owned()));
        }

        if payload.password.len() < 6 {
            return Err(AppError::BadRequest(
                "Password must be at least 6 characters".to_owned(),
            ));
        }

        let mut inn: Option<String> = None;
        if let Some(payload_inn) = payload.inn {
            inn = Some(only_digits(payload_inn));
            if !matches!(inn.as_ref().unwrap().len(), 10 | 12) {
                return Err(AppError::BadRequest("Incorrect length INN".to_owned()));
            }
        }

        let snils = Some(only_digits(payload.snils));
        if snils.as_ref().unwrap().len() != 11 {
            return Err(AppError::BadRequest("Incorrect length SNILS".to_owned()));
        }

        // Хэширование пароля
        let hash_password = bcrypt::hash(payload.password, DEFAULT_COST).map_err(|_| {
            tracing::error!("Failed to hash password in application 'create_employee'");
            AppError::BadRequest("Failed to hash password".to_owned())
        })?;

        // Валидация почты
        let correct_email = validate_email(payload.email)?;

        // Валидация номера + унификация
        let phone = phoneparse(Some(RU), payload.phone)?;

        let tariff = self.repo_default_value.get_tariff().await?.name;

        self.repo
            .save(
                payload.name,
                payload.last_name,
                payload.middle_name,
                correct_email,
                phone,
                hash_password,
                tariff,
                inn,
                snils,
            )
            .await
    }

    pub async fn get_clients(&self, filter: FilterClient) -> Vec<Client> {
        match self.repo.get(filter).await {
            Ok(vec) => vec.into_iter().map(Client::from).collect(),
            Err(_) => Vec::<Client>::new(),
        }
    }

    #[allow(dead_code)]
    pub async fn get_client(&self, email: String) -> Result<Client, AppError> {
        self.repo.get_by_email(email).await.map(Client::from)
    }

    pub async fn delete(&self, email: String) -> Result<(), AppError> {
        self.repo.delete(email).await
    }
}
