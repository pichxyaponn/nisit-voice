use crate::{
    domain::{
        entities::nisits::{NisitEntity, RegisterNisitEntity},
        repositories::nisits::NisitRepository,
    },
    infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::nisits},
};
use anyhow::Result;
use async_trait::async_trait;
use diesel::{dsl::insert_into, prelude::*};
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
        let mut connection = Arc::clone(&self.database_pool).get()?;

        let inserted = insert_into(nisits::table)
            .values(&register_nisit_entity)
            .returning(nisits::id)
            .get_result::<i32>(&mut connection)?;

        Ok(inserted)
    }

    async fn find_by_username(&self, username: &str) -> Result<NisitEntity> {
        let mut connection = Arc::clone(&self.database_pool).get()?;

        let selected = nisits::table
            .filter(nisits::username.eq(username))
            .select(NisitEntity::as_select())
            .first::<NisitEntity>(&mut connection)?;

        Ok(selected)
    }
}
