use super::{
    default_routers::{health_check, not_found},
    routers::*,
};
use crate::{
    config::config_model::DotEnvyConfig, infrastructure::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::{Ok, Result};
use axum::{Router, http::Method, routing::get};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    net::TcpListener,
    signal::unix::{SignalKind, signal},
};
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;

pub async fn serve(config: Arc<DotEnvyConfig>, database_pool: Arc<PgPoolSquad>) -> Result<()> {
    let app = create_app(config.clone(), Arc::clone(&database_pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    let listener = TcpListener::bind(addr).await?;

    // Log server startup information
    info!("🚀 Server starting on http://{}", addr);
    info!("Timeout configured for {} seconds", config.server.timeout);
    info!("Request body limit set to {} MB", config.server.body_limit);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn create_app(config: Arc<DotEnvyConfig>, database_pool: Arc<PgPoolSquad>) -> Router {
    Router::new()
        .fallback(not_found)
        .nest(
            "/report-log",
            report_log::routes(Arc::clone(&database_pool)),
        )
        .nest(
            "/report-ops",
            report_ops::routes(Arc::clone(&database_pool)),
        )
        .nest(
            "/report-assignment",
            report_assignment::routes(Arc::clone(&database_pool)),
        )
        .nest("/nisits", nisits::routes(Arc::clone(&database_pool)))
        .nest("/staff", staff::routes(Arc::clone(&database_pool)))
        .nest(
            "/report-dashboard",
            report_dashboard::routes(Arc::clone(&database_pool)),
        )
        .nest(
            "/authentication/staff",
            staff_authentication::routes(Arc::clone(&database_pool)),
        )
        .nest(
            "/authentication/nisits",
            nisits_authentication::routes(Arc::clone(&database_pool)),
        )
        .route("/health-check", get(health_check))
        .layer(TimeoutLayer::new(Duration::from_secs(
            config.server.timeout,
        )))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024)
                .try_into()
                .unwrap_or(10 * 1024 * 1024),
        ))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal(SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+C"),
        _ = terminate => info!("Received SIGTERM"),
    }
}
