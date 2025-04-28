use crate::domain::{entities::staff::StaffEntity, value_objects::staff_model::RegisterStaffModel};
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait StaffRepository {
    async fn register(&self, register_staff_model: RegisterStaffModel) -> Result<i32>;
    async fn find_by_username(&self, username: &str) -> Result<StaffEntity>;
}
