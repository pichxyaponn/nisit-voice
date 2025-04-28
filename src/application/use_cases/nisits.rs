use crate::{
    domain::{
        repositories::nisits::NisitRepository, value_objects::nisit_model::RegisterNisitModel,
    },
    infrastructure::argon2_hashing::hash_password,
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
        let hashed_password = hash_password(register_nisit_model.password.clone())?;
        register_nisit_model.password = hashed_password;

        let register_entity = register_nisit_model.to_entity();
        let staff_id = self.nisit_repository.register(register_entity).await?;

        Ok(staff_id)
    }

    pub async fn find_by_username(&self, username: &str) {
        todo!("implement find by username use case")
    }
}
