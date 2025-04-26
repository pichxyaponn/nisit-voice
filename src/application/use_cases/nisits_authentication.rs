use crate::domain::repositories::nisits::NisitRepository;
use std::sync::Arc;

pub struct NisitsAuthenticationUseCase<T>
where
    T: NisitRepository + Send + Sync,
{
    nisit_repository: Arc<T>,
}

impl<T> NisitsAuthenticationUseCase<T>
where
    T: NisitRepository + Send + Sync,
{
    pub fn new(nisit_repository: Arc<T>) -> Self {
        Self { nisit_repository }
    }

    pub async fn nisits_login(&self) {
        todo!("implement me")
    }

    pub async fn nisits_refresh_token(&self) {
        todo!("implement me")
    }
}
