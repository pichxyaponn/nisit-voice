use crate::{
    domain::entities::{reports::ReportEntity, staff::StaffEntity},
    infrastructure::postgres::schema::report_staff_junction,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, Associations)]
#[diesel(belongs_to(ReportEntity, foreign_key = report_id))]
#[diesel(belongs_to(StaffEntity, foreign_key = staff_id))]
#[diesel(table_name = report_staff_junction)]
pub struct ReportStaffJunction {
    pub report_id: i32,
    pub staff_id: i32,
}
