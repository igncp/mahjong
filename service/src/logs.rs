use crate::env::ENV_LOGGER_LEVEL;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn setup_logs() {
    let logger_level = std::env::var(ENV_LOGGER_LEVEL).unwrap_or("".to_string());

    let logger_level = match logger_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "warn" => Level::WARN,
        "info" => Level::INFO,
        "error" => Level::ERROR,
        _ => Level::WARN,
    };

    let subscriber = FmtSubscriber::builder()
        .compact()
        .with_max_level(logger_level)
        .with_ansi(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}
