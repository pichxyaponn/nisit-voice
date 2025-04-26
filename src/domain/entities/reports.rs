use crate::{
    domain::value_objects::report_model::ReportModel, infrastructure::postgres::schema::reports,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = reports)]
pub struct ReportEntity {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub nisit_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub resolved_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl ReportEntity {
    pub fn to_model(&self, staff_count: i64) -> ReportModel {
        ReportModel {
            id: self.id,
            title: self.title.clone(),
            description: self.description.clone(),
            status: self.status.clone(),
            nisit_id: self.nisit_id,
            staff_count,
            created_at: self.created_at,
            updated_at: self.updated_at,
            resolved_at: self.resolved_at,
            deleted_at: self.deleted_at,
        }
    }
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = reports)]
pub struct AddReportEntity {
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub nisit_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub resolved_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, AsChangeset, Queryable)]
#[diesel(table_name = reports)]
pub struct EditReportEntity {
    pub title: Option<String>,
    pub description: Option<String>,
    pub nisit_id: i32,
    pub updated_at: NaiveDateTime,
    pub resolved_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
