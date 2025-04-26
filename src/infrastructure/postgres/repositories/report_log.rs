use crate::{
    domain::repositories::report_log::ReportLogRepository,
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ReportLogPostgresRepository {
    database_pool: Arc<PgPoolSquad>,
}

impl ReportLogPostgresRepository {
    pub fn new(database_pool: Arc<PgPoolSquad>) -> Self {
        Self { database_pool }
    }
}

#[async_trait]
impl ReportLogRepository for ReportLogPostgresRepository {
    async fn in_progress(&self, report_id: i32, nisit_id: i32) -> Result<i32> {
        todo!("implement me")
    }

    async fn to_closed(&self, report_id: i32, nisit_id: i32) -> Result<i32> {
        todo!("implement me")
    }

    async fn to_canceled(&self, report_id: i32, nisit_id: i32) -> Result<i32> {
        todo!("implement me")
    }
}
