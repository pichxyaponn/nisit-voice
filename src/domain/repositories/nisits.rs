use crate::domain::entities::nisits::{NisitEntity, RegisterNisitEntity};
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait NisitRepository {
    async fn register(&self, register_nisit_entity: RegisterNisitEntity) -> Result<i32>;
    async fn find_by_username(&self, username: &str) -> Result<NisitEntity>;
}
