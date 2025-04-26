use crate::{
    domain::{
        entities::reports::ReportEntity, repositories::report_dashboard::ReportDashboardRepository,
        value_objects::board_checking_filter::BoardCheckingFilter,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ReportDashboardPostgresRepository {
    database_pool: Arc<PgPoolSquad>,
}

impl ReportDashboardPostgresRepository {
    pub fn new(database_pool: Arc<PgPoolSquad>) -> Self {
        Self { database_pool }
    }
}

#[async_trait]
impl ReportDashboardRepository for ReportDashboardPostgresRepository {
    async fn view_details(&self, report_id: i32) -> Result<ReportEntity> {
        todo!("implement me")
    }

    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<ReportEntity>> {
        todo!("implement me")
    }

    async fn staff_counting_by_report_id(&self, report_id: i32) -> Result<i64> {
        todo!("implement me")
    }
}
