use crate::domain::entities::staff::{RegisterStaffEntity, StaffEntity};
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait StaffRepository {
    async fn register(&self, register_staff_entity: RegisterStaffEntity) -> Result<i32>;
    async fn find_by_username(&self, username: &str) -> Result<StaffEntity>;
}
