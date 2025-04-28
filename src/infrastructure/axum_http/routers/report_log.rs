use crate::{
    application::use_cases::report_log::ReportLogUseCase,
    domain::repositories::{
        report_dashboard::ReportDashboardRepository, report_log::ReportLogRepository,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            report_dashboard::ReportDashboardPostgresRepository,
            report_log::ReportLogPostgresRepository,
        },
    },
};
use axum::{
    Extension, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::patch,
};
use std::sync::Arc;

pub fn routes(database_pool: Arc<PgPoolSquad>) -> Router {
    let report_log_repository = ReportLogPostgresRepository::new(Arc::clone(&database_pool));
    let report_dashboard_repository =
        ReportDashboardPostgresRepository::new(Arc::clone(&database_pool));

    let report_log_usecase = ReportLogUseCase::new(
        Arc::new(report_log_repository),
        Arc::new(report_dashboard_repository),
    );

    Router::new()
        .route("/in-progress/{:report_id}", patch(in_progress))
        .route("/to-closed/{:report_id}", patch(to_closed))
        .route("/to-canceled/{:report_id}", patch(to_canceled))
        .with_state(Arc::new(report_log_usecase))
}

pub async fn in_progress<T1, T2>(
    State(report_log_usecase): State<Arc<ReportLogUseCase<T1, T2>>>,
    Extension(nisit_id): Extension<i32>,
    Path(report_id): Path<i32>,
) -> impl IntoResponse
where
    T1: ReportLogRepository + Send + Sync,
    T2: ReportDashboardRepository + Send + Sync,
{
    // Implement the in_progress logic here
    "In progress staff"
}

pub async fn to_closed<T1, T2>(
    State(report_log_usecase): State<Arc<ReportLogUseCase<T1, T2>>>,
    Extension(nisit_id): Extension<i32>,
    Path(report_id): Path<i32>,
) -> impl IntoResponse
where
    T1: ReportLogRepository + Send + Sync,
    T2: ReportDashboardRepository + Send + Sync,
{
    // Implement the to_closed logic here
    "Report closed"
}

pub async fn to_canceled<T1, T2>(
    State(report_log_usecase): State<Arc<ReportLogUseCase<T1, T2>>>,
    Extension(nisit_id): Extension<i32>,
    Path(report_id): Path<i32>,
) -> impl IntoResponse
where
    T1: ReportLogRepository + Send + Sync,
    T2: ReportDashboardRepository + Send + Sync,
{
    // Implement the to_canceled logic here
    "Report canceled"
}
