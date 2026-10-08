use crate::system::config::{AppConfig, env::AppEnv};
use time::{UtcOffset, macros::format_description};
use tracing_subscriber::{
    EnvFilter,
    fmt::{layer, time::OffsetTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub fn init_tracing(config: &AppConfig) {
    let default_filter = match config.app_env {
        AppEnv::Development => "portfolio=debug,tower_http=debug,sqlx=info",
        AppEnv::Staging => "portfolio=debug,tower_http=info,sqlx=info",
        AppEnv::Production => "portfolio=info,tower_http=info,sqlx=warn",
    };
    let format =
        format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]");
    let timer = OffsetTime::new(
        UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC),
        format,
    );
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));

    match config.app_env {
        AppEnv::Development => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    layer()
                        .compact()
                        .with_ansi(true)
                        .with_timer(timer)
                        .with_target(true)
                        .with_writer(std::io::stdout),
                )
                .init();
        }
        AppEnv::Staging | AppEnv::Production => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    layer()
                        .json()
                        .with_ansi(false)
                        .with_target(true)
                        .with_current_span(true)
                        .with_writer(std::io::stdout),
                )
                .init();
        }
    };
}
