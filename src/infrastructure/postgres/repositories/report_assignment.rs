use crate::{
    domain::{
        repositories::report_assignment::ReportAssignmentRepository,
        value_objects::report_staff_junction::ReportStaffJunction,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ReportAssignmentPostgresRepository {
    database_pool: Arc<PgPoolSquad>,
}

impl ReportAssignmentPostgresRepository {
    pub fn new(database_pool: Arc<PgPoolSquad>) -> Self {
        Self { database_pool }
    }
}

#[async_trait]
impl ReportAssignmentRepository for ReportAssignmentPostgresRepository {
    async fn assign(&self, junction_body: ReportStaffJunction) -> Result<()> {
        todo!("implement me")
    }

    async fn unassign(&self, junction_body: ReportStaffJunction) -> Result<()> {
        todo!("implement me")
    }
}
