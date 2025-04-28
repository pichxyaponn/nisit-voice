use crate::domain::{
    entities::nisits::NisitEntity, value_objects::nisit_model::RegisterNisitModel,
};
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait NisitRepository {
    async fn register(&self, register_nisit_model: RegisterNisitModel) -> Result<i32>;
    async fn find_by_username(&self, username: &str) -> Result<NisitEntity>;
}
