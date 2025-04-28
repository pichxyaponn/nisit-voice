use crate::{
    domain::{
        repositories::report_ops::ReportOpsRepository, value_objects::report_model::AddReportModel,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

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
    async fn add(&self, add_report_model: AddReportModel) -> Result<i32> {
        todo!("implement me")
    }

    async fn edit(&self, quest_id: i32, edit_report_model: AddReportModel) -> Result<i32> {
        todo!("implement me")
    }

    async fn remove(&self, quest_id: i32, staff_id: i32) -> Result<()> {
        todo!("implement me")
    }
}
