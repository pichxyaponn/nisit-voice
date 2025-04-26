use crate::domain::value_objects::report_staff_junction::ReportStaffJunction;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait ReportAssignmentRepository {
    async fn assign(&self, junction_body: ReportStaffJunction) -> Result<()>;
    async fn unassign(&self, junction_body: ReportStaffJunction) -> Result<()>;
}
