use crate::domain::{
    repositories::staff::StaffRepository, value_objects::staff_model::RegisterStaffModel,
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
        todo!("implement register staff use case")
    }

    pub async fn find_by_username(&self, username: &str) {
        todo!("implement find by username use case")
    }
}
