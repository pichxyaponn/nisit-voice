use crate::{
    application::use_cases::nisits_authentication::NisitsAuthenticationUseCase,
    domain::repositories::nisits::NisitRepository,
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::nisits::NisitPostgresRepository,
    },
};
use axum::{Router, extract::State, response::IntoResponse, routing::post};
use std::sync::Arc;

pub fn routes(database_pool: Arc<PgPoolSquad>) -> Router {
    let nisits_repository = NisitPostgresRepository::new(database_pool);
    let nisits_authentication_usecase =
        NisitsAuthenticationUseCase::new(Arc::new(nisits_repository));

    Router::new()
        .route("/login", post(nisits_login))
        .route("/refresh-token", post(nisits_refresh_token))
        .with_state(Arc::new(nisits_authentication_usecase))
}

pub async fn nisits_login<T>(
    State(nisits_usecase): State<Arc<NisitsAuthenticationUseCase<T>>>,
) -> impl IntoResponse
where
    T: NisitRepository + Send + Sync,
{
    // Implement the login logic here
    "Login Nisit"
}

pub async fn nisits_refresh_token<T>(
    State(nisits_usecase): State<Arc<NisitsAuthenticationUseCase<T>>>,
) -> impl IntoResponse
where
    T: NisitRepository + Send + Sync,
{
    // Implement the refresh token logic here
    "Refresh Token Nisit"
}
