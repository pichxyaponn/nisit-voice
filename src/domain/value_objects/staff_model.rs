use crate::domain::entities::staff::RegisterStaffEntity;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterStaffModel {
    pub username: String,
    pub password: String,
}

impl RegisterStaffModel {
    pub fn to_entity(&self) -> RegisterStaffEntity {
        RegisterStaffEntity {
            username: self.username.clone(),
            password: self.password.clone(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
