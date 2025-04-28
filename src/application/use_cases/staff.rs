use crate::{
    domain::{
        repositories::staff::StaffRepository, value_objects::staff_model::RegisterStaffModel,
    },
    infrastructure::argon2_hashing::hash_password,
};
use anyhow::Result;
use std::sync::Arc;

pub struct StaffUseCase<T>
where
    T: StaffRepository + Send + Sync,
{
    staff_repository: Arc<T>,
}

impl<T> StaffUseCase<T>
where
    T: StaffRepository + Send + Sync,
{
    pub fn new(staff_repository: Arc<T>) -> Self {
        Self { staff_repository }
    }

    pub async fn register(&self, mut register_staff_model: RegisterStaffModel) -> Result<i32> {
        let hashed_password = hash_password(register_staff_model.password.clone())?;
        register_staff_model.password = hashed_password;

        let register_entity = register_staff_model.to_entity();
        let staff_id = self.staff_repository.register(register_entity).await?;

        Ok(staff_id)
    }

    pub async fn find_by_username(&self, username: &str) {
        todo!("implement find by username use case")
    }
}
