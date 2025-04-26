use crate::domain::entities::reports::AddReportEntity;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait ReportOpsRepository {
    async fn add(&self, add_report_entity: AddReportEntity) -> Result<i32>;
    async fn edit(&self, quest_id: i32, edit_report_entity: AddReportEntity) -> Result<i32>;
    async fn remove(&self, quest_id: i32, staff_id: i32) -> Result<()>;
}
