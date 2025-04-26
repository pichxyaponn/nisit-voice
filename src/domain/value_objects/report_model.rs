use super::report_status::ReportStatus;
use crate::domain::entities::reports::{AddReportEntity, EditReportEntity};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportModel {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub nisit_id: i32,
    pub staff_count: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub resolved_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddReportModel {
    pub title: String,
    pub description: Option<String>,
}

impl AddReportModel {
    pub fn to_entity(&self, nisit_id: i32) -> AddReportEntity {
        AddReportEntity {
            title: self.title.clone(),
            description: self.description.clone(),
            nisit_id,
            status: ReportStatus::Open.to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            resolved_at: None,
            deleted_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditReportModel {
    pub title: Option<String>,
    pub description: Option<String>,
}

impl EditReportModel {
    pub fn to_entity(&self, nisit_id: i32) -> EditReportEntity {
        EditReportEntity {
            title: self.title.clone(),
            description: self.description.clone(),
            nisit_id,
            updated_at: Utc::now().naive_utc(),
            resolved_at: None,
            deleted_at: None,
        }
    }
}
