use crate::{
    application::use_cases::report_assignment::ReportAssignmentUseCase,
    domain::repositories::{
        report_assignment::ReportAssignmentRepository, report_dashboard::ReportDashboardRepository,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            report_assignment::ReportAssignmentPostgresRepository,
            report_dashboard::ReportDashboardPostgresRepository,
        },
    },
};
use axum::{
    Extension, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, post},
};
use std::sync::Arc;

pub fn routes(database_pool: Arc<PgPoolSquad>) -> Router {
    let report_assignment_repository =
        ReportAssignmentPostgresRepository::new(Arc::clone(&database_pool));
    let report_dashboard_repository =
        ReportDashboardPostgresRepository::new(Arc::clone(&database_pool));

    let report_assignment_usecase = ReportAssignmentUseCase::new(
        Arc::new(report_assignment_repository),
        Arc::new(report_dashboard_repository),
    );

    Router::new()
        .route("/join/{:report_id}", post(join))
        .route("/leave/{:report_id}", delete(leave))
        .with_state(Arc::new(report_assignment_usecase))
}

pub async fn join<T1, T2>(
    State(report_assignment_usecase): State<Arc<ReportAssignmentUseCase<T1, T2>>>,
    Extension(staff_id): Extension<i32>,
    Path(report_id): Path<i32>,
) -> impl IntoResponse
where
    T1: ReportAssignmentRepository + Send + Sync,
    T2: ReportDashboardRepository + Send + Sync,
{
    // Implement the join logic here
    "Join staff"
}

pub async fn leave<T1, T2>(
    State(report_assignment_usecase): State<Arc<ReportAssignmentUseCase<T1, T2>>>,
    Extension(staff_id): Extension<i32>,
    Path(report_id): Path<i32>,
) -> impl IntoResponse
where
    T1: ReportAssignmentRepository + Send + Sync,
    T2: ReportDashboardRepository + Send + Sync,
{
    // Implement the leave logic here
    "Leave staff"
}
