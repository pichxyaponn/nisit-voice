use crate::{
    application::use_cases::staff_authentication::StaffAuthenticationUseCase,
    domain::repositories::staff::StaffRepository,
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::staff::StaffPostgresRepository,
    },
};
use axum::{Router, extract::State, response::IntoResponse, routing::post};
use std::sync::Arc;

pub fn routes(database_pool: Arc<PgPoolSquad>) -> Router {
    let staff_repository = StaffPostgresRepository::new(database_pool);
    let staff_authentication_usecase = StaffAuthenticationUseCase::new(Arc::new(staff_repository));

    Router::new()
        .route("/login", post(staff_login))
        .route("/refresh-token", post(staff_refresh_token))
        .with_state(Arc::new(staff_authentication_usecase))
}

pub async fn staff_login<T>(
    State(staff_usecase): State<Arc<StaffAuthenticationUseCase<T>>>,
) -> impl IntoResponse
where
    T: StaffRepository + Send + Sync,
{
    // Implement the login logic here
    "Login Staff"
}

pub async fn staff_refresh_token<T>(
    State(staff_usecase): State<Arc<StaffAuthenticationUseCase<T>>>,
) -> impl IntoResponse
where
    T: StaffRepository + Send + Sync,
{
    // Implement the refresh token logic here
    "Refresh Token Staff"
}
