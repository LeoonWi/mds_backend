use dotenv::dotenv;
use std::{env, error::Error};

/// Структура `Config` хранит конфигурационные параметры приложения, загружаемые из переменных окружения.
///
/// ## Поля
/// Подробности о полях см. ниже.
///
/// ## Пример использования
/// ```ignore
/// let config = Config::build().unwrap();
/// println!("Status: {}", config.status);
/// ```
pub struct Config {
    /// `profile` Режим работы приложения `"dev"` (разработка) или `"prod"` (продакшен).
    ///
    /// Ожидает переменную окружения `PROFILE`.
    ///
    /// ## Пример
    /// `RUST_STATUS=prod`
    pub profile: String,

    /// `db_url` Строка подключения к базе данных Postgres.
    ///
    /// Ожидает переменные окружения `POSTGRES_USER`, `POSTGRES_PASSWORD`, `POSTGRES_HOST`, `POSTGRES_PORT`, `POSTGRES_DB` и опционально `POSTGRES_ARGS`
    ///
    /// ## Пример
    /// `POSTGRES_USER=postgres`
    /// `POSTGRES_PASSWORD=admin`
    /// `POSTGRES_HOST=localhost`
    /// `POSTGRES_PORT=5433`
    /// `POSTGRES_DB=mds`
    /// `POSTGRES_ARGS=sslmode=disable`
    pub db_url: String,

    /// `ip` IP адрес на котором запускается сервер (IPv4).
    ///
    /// Ожидает переменную окружения `SERVER_IP`.
    ///
    /// ## Пример
    /// `SERVER_IP=127.0.0.1`
    pub ip: String,

    /// `port` Порт на котором запускается сервер.
    ///
    /// Ожидает переменную окружения `SERVER_PORT`.
    ///
    /// ## Пример
    /// `SERVER_PORT=3000`
    pub port: String,
}

impl Config {
    pub fn build() -> Result<Config, Box<dyn Error>> {
        dotenv().ok();

        let database_url;
        {
            let user = env::var("POSTGRES_USER")?;
            let password = env::var("POSTGRES_PASSWORD")?;
            let host = env::var("POSTGRES_HOST")?;
            let port = env::var("POSTGRES_PORT")?;
            let db = env::var("POSTGRES_DB")?;
            let args = match env::var("POSTGRES_ARGS") {
                Ok(string) => "?".to_string() + &string,
                Err(_) => "".to_string(),
            };

            database_url = format!(
                "postgres://{}:{}@{}:{}/{}{}",
                user, password, host, port, db, args
            );
        }

        let ip = env::var("SERVER_HOST")?;
        let port = env::var("SERVER_PORT")?;
        let profile = match env::var("PROFILE") {
            Ok(v) => v,
            Err(_) => "".to_string(),
        };

        Ok(Config {
            profile: profile,
            ip: ip,
            db_url: database_url,
            port: port,
        })
    }
}
