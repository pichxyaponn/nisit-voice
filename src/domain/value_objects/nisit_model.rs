use crate::domain::entities::nisits::RegisterNisitEntity;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterNisitModel {
    pub username: String,
    pub password: String,
}

impl RegisterNisitModel {
    pub fn to_entity(&self) -> RegisterNisitEntity {
        RegisterNisitEntity {
            username: self.username.clone(),
            password: self.password.clone(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
