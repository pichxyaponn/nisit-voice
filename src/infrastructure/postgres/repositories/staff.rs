use crate::{
    domain::{
        entities::staff::{RegisterStaffEntity, StaffEntity},
        repositories::staff::StaffRepository,
    },
    infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::staff},
};
use anyhow::Result;
use async_trait::async_trait;
use diesel::{dsl::insert_into, prelude::*};
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
    async fn register(&self, register_staff_entity: RegisterStaffEntity) -> Result<i32> {
        let mut connection = Arc::clone(&self.database_pool).get()?;

        let inserted = insert_into(staff::table)
            .values(&register_staff_entity)
            .returning(staff::id)
            .get_result::<i32>(&mut connection)?;

        Ok(inserted)
    }

    async fn find_by_username(&self, username: &str) -> Result<StaffEntity> {
        let mut connection = Arc::clone(&self.database_pool).get()?;

        let selected = staff::table
            .filter(staff::username.eq(username))
            .select(StaffEntity::as_select())
            .first::<StaffEntity>(&mut connection)?;

        Ok(selected)
    }
}
