use crate::domain::{
    repositories::nisits::NisitRepository, value_objects::nisit_model::RegisterNisitModel,
};
use anyhow::Result;
use std::sync::Arc;

pub struct NisitUseCase<T>
where
    T: NisitRepository + Send + Sync,
{
    nisit_repository: Arc<T>,
}

impl<T> NisitUseCase<T>
where
    T: NisitRepository + Send + Sync,
{
    pub fn new(nisit_repository: Arc<T>) -> Self {
        Self { nisit_repository }
    }

    pub async fn register(&self, mut register_nisit_model: RegisterNisitModel) -> Result<i32> {
        todo!("implement register nisit use case")
    }

    pub async fn find_by_username(&self, username: &str) {
        todo!("implement find by username use case")
    }
}
