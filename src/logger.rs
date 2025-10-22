use tracing_subscriber::{
    EnvFilter,
    fmt::{self},
    prelude::*,
};

// Уровень логгера задаётся из .env с поля RUST_LOG.
// Возможные значения:
// RUST_LOG=info: Только info, warn, error;
// RUST_LOG=debug: Добавляет debug;
// RUST_LOG=crate::features::services=trace: Только trace для модуля services.

// Использование:
// tracing::error!: Ошибки (например, сбой БД);
// tracing::warn!: Предупреждения (например, некорректные входные данные);
// tracing::info!: Важные события (например, запуск сервера, создание записи);
// tracing::debug!: Детали для отладки (например, входные параметры);
// tracing::trace!: Очень подробные логи.

pub fn init_dev_logger() {
    // Настройка формата логов: время, уровень, модуль
    let fmt_layer = fmt::layer().with_line_number(true);

    // Читаем уровень логов из переменной окружения RUST_LOG (например, "info", "debug")
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Устанавливаем глобальный subscriber
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}

pub fn init_prod_logger() {
    // JSON-формат для продакшена
    let fmt_layer = fmt::layer()
        .json() // Вывод в JSON
        .with_line_number(true);

    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
