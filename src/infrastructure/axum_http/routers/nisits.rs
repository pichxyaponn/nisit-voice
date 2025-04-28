use crate::{
    application::use_cases::nisits::NisitUseCase,
    domain::{
        repositories::nisits::NisitRepository, value_objects::nisit_model::RegisterNisitModel,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::nisits::NisitPostgresRepository,
    },
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use std::sync::Arc;

pub fn routes(database_pool: Arc<PgPoolSquad>) -> Router {
    let nisits_repository = NisitPostgresRepository::new(database_pool);
    let nisits_usecase = NisitUseCase::new(Arc::new(nisits_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(nisits_usecase))
}

pub async fn register<T>(
    State(nisits_usecase): State<Arc<NisitUseCase<T>>>,
    Json(register_nisit_model): Json<RegisterNisitModel>,
) -> impl IntoResponse
where
    T: NisitRepository + Send + Sync,
{
    match nisits_usecase.register(register_nisit_model).await {
        Ok(nisits_id) => (
            StatusCode::CREATED,
            format!("Nisits registered with ID: {}", nisits_id),
        )
            .into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", err)).into_response(),
    }
}
