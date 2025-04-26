use core::fmt::Display;
use serde::{Deserialize, Serialize};
use std::fmt::{Formatter, Result};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]

pub enum ReportStatus {
    #[default]
    Open,
    InProgress,
    Closed,
    Cancelled,
}

impl Display for ReportStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let status = match self {
            ReportStatus::Open => "Open",
            ReportStatus::InProgress => "In Progress",
            ReportStatus::Closed => "Closed",
            ReportStatus::Cancelled => "Cancelled",
        };
        write!(f, "{}", status)
    }
}
