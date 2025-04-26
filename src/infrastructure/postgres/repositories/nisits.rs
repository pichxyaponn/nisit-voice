use crate::{
    domain::{
        entities::nisits::{NisitEntity, RegisterNisitEntity},
        repositories::nisits::NisitRepository,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct NisitPostgresRepository {
    database_pool: Arc<PgPoolSquad>,
}

impl NisitPostgresRepository {
    pub fn new(database_pool: Arc<PgPoolSquad>) -> Self {
        Self { database_pool }
    }
}

#[async_trait]
impl NisitRepository for NisitPostgresRepository {
    async fn register(&self, register_nisit_entity: RegisterNisitEntity) -> Result<i32> {
        todo!("implement me")
    }

    async fn find_by_username(&self, username: &str) -> Result<NisitEntity> {
        todo!("implement me")
    }
}
