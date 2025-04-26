use crate::domain::{
    repositories::{report_dashboard::ReportDashboardRepository, report_ops::ReportOpsRepository},
    value_objects::report_model::{AddReportModel, EditReportModel},
};
use anyhow::Result;
use std::sync::Arc;

pub struct ReportOpsUseCase<T1, T2>
where
    T1: ReportOpsRepository,
    T2: ReportDashboardRepository,
{
    report_ops_repository: Arc<T1>,
    report_dashboard_repository: Arc<T2>,
}

impl<T1, T2> ReportOpsUseCase<T1, T2>
where
    T1: ReportOpsRepository + Send + Sync,
    T2: ReportDashboardRepository + Send + Sync,
{
    pub fn new(report_ops_repository: Arc<T1>, report_dashboard_repository: Arc<T2>) -> Self {
        Self {
            report_ops_repository,
            report_dashboard_repository,
        }
    }

    pub async fn add(&self, nisit_id: i32, add_report_model: AddReportModel) -> Result<i32> {
        todo!("implement me")
    }

    pub async fn edit(
        &self,
        report_id: i32,
        nisit_id: i32,
        edit_report_entity: EditReportModel,
    ) -> Result<i32> {
        todo!("implement me")
    }

    pub async fn remove(&self, report_id: i32, nisit_id: i32) -> Result<()> {
        todo!("implement me")
    }
}
