use crate::{
    application::use_cases::report_dashboard::ReportDashboardUseCase,
    domain::{
        repositories::report_dashboard::ReportDashboardRepository,
        value_objects::board_checking_filter::BoardCheckingFilter,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::report_dashboard::ReportDashboardPostgresRepository,
    },
};
use axum::{
    Router,
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::post,
};
use std::sync::Arc;

pub fn routes(database_pool: Arc<PgPoolSquad>) -> Router {
    let report_dashboard_repository = ReportDashboardPostgresRepository::new(database_pool);
    let report_dashboard_usecase =
        ReportDashboardUseCase::new(Arc::new(report_dashboard_repository));

    Router::new()
        .route("/{:report_id}", post(view_details))
        .route("/board-checking", post(board_checking))
        .with_state(Arc::new(report_dashboard_usecase))
}

pub async fn view_details<T>(
    State(report_dashboard_usecase): State<Arc<ReportDashboardUseCase<T>>>,
    Path(report_id): Path<i32>,
) -> impl IntoResponse
where
    T: ReportDashboardRepository + Send + Sync,
{
    // Implement the view details logic here
    "View report details"
}

pub async fn board_checking<T>(
    State(report_assignment_usecase): State<Arc<ReportDashboardUseCase<T>>>,
    filter: Query<BoardCheckingFilter>,
) -> impl IntoResponse
where
    T: ReportDashboardRepository + Send + Sync,
{
    // Implement the board checking logic here
    "Board checking"
}
