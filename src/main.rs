use nisit_voice::{config, infrastructure::postgres::postgres_connection};
use tracing::{info, level_filters::LevelFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let dotenvy_config = match config::config_loader::load() {
        Ok(config) => config,
        Err(e) => {
            tracing::error!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    info!("Loaded .env config");

    let _ = match postgres_connection::establish_connection(&dotenvy_config.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("Failed to establish connection to database: {}", e);
            std::process::exit(1);
        }
    };

    info!("Established connection to database");
}
