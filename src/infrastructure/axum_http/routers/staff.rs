use crate::{
    application::use_cases::staff::StaffUseCase,
    domain::{
        repositories::staff::StaffRepository, value_objects::staff_model::RegisterStaffModel,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::staff::StaffPostgresRepository,
    },
};
use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use std::sync::Arc;

pub fn routes(database_pool: Arc<PgPoolSquad>) -> Router {
    let staff_repository = StaffPostgresRepository::new(database_pool);
    let staff_usecase = StaffUseCase::new(Arc::new(staff_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(staff_usecase))
}

pub async fn register<T>(
    State(staff_usecase): State<Arc<StaffUseCase<T>>>,
    Json(register_staff_model): Json<RegisterStaffModel>,
) -> impl IntoResponse
where
    T: StaffRepository + Send + Sync,
{
    // Implement the registration logic here
    "Register staff"
}
