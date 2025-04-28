use crate::domain::value_objects::report_model::AddReportModel;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait ReportOpsRepository {
    async fn add(&self, add_report_model: AddReportModel) -> Result<i32>;
    async fn edit(&self, quest_id: i32, edit_report_model: AddReportModel) -> Result<i32>;
    async fn remove(&self, quest_id: i32, staff_id: i32) -> Result<()>;
}
