use super::report_status::ReportStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardCheckingFilter {
    pub name: Option<String>,
    pub status: Option<ReportStatus>,
}
