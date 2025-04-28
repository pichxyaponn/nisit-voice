use crate::{
    domain::{
        entities::staff::StaffEntity, repositories::staff::StaffRepository,
        value_objects::staff_model::RegisterStaffModel,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct StaffPostgresRepository {
    database_pool: Arc<PgPoolSquad>,
}

impl StaffPostgresRepository {
    pub fn new(database_pool: Arc<PgPoolSquad>) -> Self {
        Self { database_pool }
    }
}

#[async_trait]
impl StaffRepository for StaffPostgresRepository {
    async fn register(&self, register_staff_model: RegisterStaffModel) -> Result<i32> {
        todo!("implement me")
    }

    async fn find_by_username(&self, username: &str) -> Result<StaffEntity> {
        todo!("implement me")
    }
}
