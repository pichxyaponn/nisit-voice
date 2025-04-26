use crate::domain::repositories::{
    report_assignment::ReportAssignmentRepository, report_dashboard::ReportDashboardRepository,
};
use anyhow::Result;
use std::sync::Arc;

pub struct ReportAssignmentUseCase<T1, T2>
where
    T1: ReportAssignmentRepository + Send + Sync,
    T2: ReportDashboardRepository + Send + Sync,
{
    report_assignment_repository: Arc<T1>,
    report_dashboard_repository: Arc<T2>,
}

impl<T1, T2> ReportAssignmentUseCase<T1, T2>
where
    T1: ReportAssignmentRepository + Send + Sync,
    T2: ReportDashboardRepository + Send + Sync,
{
    pub fn new(
        report_assignment_repository: Arc<T1>,
        report_dashboard_repository: Arc<T2>,
    ) -> Self {
        Self {
            report_assignment_repository,
            report_dashboard_repository,
        }
    }

    pub async fn assign(&self, report_id: i32, staff_id: i32) -> Result<()> {
        todo!("implement me")
    }

    pub async fn unassign(&self, report_id: i32, staff_id: i32) -> Result<()> {
        todo!("implement me")
    }
}
