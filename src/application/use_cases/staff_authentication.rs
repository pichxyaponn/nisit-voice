use crate::domain::repositories::staff::StaffRepository;
use std::sync::Arc;

pub struct StaffAuthenticationUseCase<T>
where
    T: StaffRepository + Send + Sync,
{
    staff_repository: Arc<T>,
}

impl<T> StaffAuthenticationUseCase<T>
where
    T: StaffRepository + Send + Sync,
{
    pub fn new(staff_repository: Arc<T>) -> Self {
        Self { staff_repository }
    }

    pub async fn staff_login(&self) {
        todo!("implement me")
    }

    pub async fn staff_refresh_token(&self) {
        todo!("implement me")
    }
}
