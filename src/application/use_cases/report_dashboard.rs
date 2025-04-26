use crate::domain::{
    repositories::report_dashboard::ReportDashboardRepository,
    value_objects::{board_checking_filter::BoardCheckingFilter, report_model::ReportModel},
};
use anyhow::Result;
use std::sync::Arc;

pub struct ReportDashboardUseCase<T>
where
    T: ReportDashboardRepository + Send + Sync,
{
    report_dashboard_repository: Arc<T>,
}

impl<T> ReportDashboardUseCase<T>
where
    T: ReportDashboardRepository + Send + Sync,
{
    pub fn new(report_dashboard_repository: Arc<T>) -> Self {
        Self {
            report_dashboard_repository,
        }
    }

    pub async fn view_details(&self, report_id: i32) -> Result<ReportModel> {
        todo!("implement me")
    }

    pub async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<ReportModel>> {
        todo!("implement me")
    }

    pub async fn staff_counting_by_report_id(&self, report_id: i32) -> Result<()> {
        todo!("implement me")
    }
}
