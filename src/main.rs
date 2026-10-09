use portfolio::app::error::AppError;
use portfolio::system::state::AppState;
use portfolio::system::{config::AppConfig, state::telemetry::init_tracing};
use portfolio::web;
use tracing::error;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = AppConfig::new(".env")?;
    init_tracing(&config);
    let state = AppState::new(config).await.map_err(AppError::StateError)?;
    if let Err(err) = web::server::run(state).await {
        error!(error = ?err, "Application exited with error");
    }
    Ok(())
}
