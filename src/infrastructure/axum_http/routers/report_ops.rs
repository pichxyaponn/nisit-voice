use crate::{
    application::use_cases::report_ops::ReportOpsUseCase,
    domain::{
        repositories::{
            report_dashboard::ReportDashboardRepository, report_ops::ReportOpsRepository,
        },
        value_objects::report_model::ReportModel,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            report_dashboard::ReportDashboardPostgresRepository,
            report_ops::ReportOpsPostgresRepository,
        },
    },
};
use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, patch, post},
};
use std::sync::Arc;

pub fn routes(database_pool: Arc<PgPoolSquad>) -> Router {
    let report_ops_repository = ReportOpsPostgresRepository::new(Arc::clone(&database_pool));
    let report_dashboard_repository =
        ReportDashboardPostgresRepository::new(Arc::clone(&database_pool));

    let report_ops_usecase = ReportOpsUseCase::new(
        Arc::new(report_ops_repository),
        Arc::new(report_dashboard_repository),
    );

    Router::new()
        .route("/", post(add))
        .route("/{:report_id}", patch(edit))
        .route("/{:report_id}", delete(remove))
        .with_state(Arc::new(report_ops_usecase))
}

pub async fn add<T1, T2>(
    State(report_ops_usecase): State<Arc<ReportOpsUseCase<T1, T2>>>,
    Extension(nisit_id): Extension<i32>,
    Json(report_model): Json<ReportModel>,
) -> impl IntoResponse
where
    T1: ReportOpsRepository + Send + Sync,
    T2: ReportDashboardRepository + Send + Sync,
{
    // Implement the add logic here using nisit_id
    "Report added"
}

pub async fn edit<T1, T2>(
    State(report_ops_usecase): State<Arc<ReportOpsUseCase<T1, T2>>>,
    Path(report_id): Path<i32>,
    Extension(nisit_id): Extension<i32>,
    Json(report_model): Json<ReportModel>,
) -> impl IntoResponse
where
    T1: ReportOpsRepository + Send + Sync,
    T2: ReportDashboardRepository + Send + Sync,
{
    // Implement the edit logic here using nisit_id and report_id
    "Report edited"
}

pub async fn remove<T1, T2>(
    State(report_ops_usecase): State<Arc<ReportOpsUseCase<T1, T2>>>,
    Path(report_id): Path<i32>,
    Extension(nisit_id): Extension<i32>,
) -> impl IntoResponse
where
    T1: ReportOpsRepository + Send + Sync,
    T2: ReportDashboardRepository + Send + Sync,
{
    // Implement the remove logic here using nisit_id and report_id
    "Report removed"
}
