use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::{entities::reports::AddReportEntity, repositories::report_ops::ReportOpsRepository},
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct ReportOpsPostgresRepository {
    database_pool: Arc<PgPoolSquad>,
}

impl ReportOpsPostgresRepository {
    pub fn new(database_pool: Arc<PgPoolSquad>) -> Self {
        Self { database_pool }
    }
}

#[async_trait]
impl ReportOpsRepository for ReportOpsPostgresRepository {
    async fn add(&self, add_report_entity: AddReportEntity) -> Result<i32> {
        todo!("implement me")
    }

    async fn edit(&self, quest_id: i32, edit_report_entity: AddReportEntity) -> Result<i32> {
        todo!("implement me")
    }

    async fn remove(&self, quest_id: i32, staff_id: i32) -> Result<()> {
        todo!("implement me")
    }
}
