use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait ReportLogRepository {
    async fn in_progress(&self, report_id: i32, nisit_id: i32) -> Result<i32>;
    async fn to_closed(&self, report_id: i32, nisit_id: i32) -> Result<i32>;
    async fn to_canceled(&self, report_id: i32, nisit_id: i32) -> Result<i32>;
}
