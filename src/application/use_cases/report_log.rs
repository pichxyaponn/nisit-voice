use crate::domain::repositories::{
    report_dashboard::ReportDashboardRepository, report_log::ReportLogRepository,
};
use anyhow::Result;
use std::sync::Arc;

pub struct ReportLogUseCase<T1, T2>
where
    T1: ReportLogRepository + Send + Sync,
    T2: ReportDashboardRepository + Send + Sync,
{
    report_log_repository: Arc<T1>,
    report_dashboard_repository: Arc<T2>,
}

impl<T1, T2> ReportLogUseCase<T1, T2>
where
    T1: ReportLogRepository + Send + Sync,
    T2: ReportDashboardRepository + Send + Sync,
{
    pub fn new(report_log_repository: Arc<T1>, report_dashboard_repository: Arc<T2>) -> Self {
        Self {
            report_log_repository,
            report_dashboard_repository,
        }
    }

    pub async fn in_progress(&self, report_id: i32, nisit_id: i32) -> Result<i32> {
        todo!("implement me")
    }

    pub async fn to_closed(&self, report_id: i32, nisit_id: i32) -> Result<i32> {
        todo!("implement me")
    }

    pub async fn to_canceled(&self, report_id: i32, nisit_id: i32) -> Result<i32> {
        todo!("implement me")
    }
}
