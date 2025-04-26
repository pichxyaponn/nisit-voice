use crate::domain::{
    entities::reports::ReportEntity, value_objects::board_checking_filter::BoardCheckingFilter,
};
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait ReportDashboardRepository {
    async fn view_details(&self, report_id: i32) -> Result<ReportEntity>;
    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<ReportEntity>>;
    async fn staff_counting_by_report_id(&self, report_id: i32) -> Result<i64>;
}
