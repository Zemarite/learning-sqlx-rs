use crate::domain::entities::Organization;
use crate::domain::errors::Result;
use crate::domain::value_object::OrganizationId;
use async_trait::async_trait;

#[async_trait]
pub trait OrganizationRepository {
    async fn find_all(&self) -> Result<Vec<Organization>>;
    async fn find_by_id(&self, id: OrganizationId) -> Result<Option<Organization>>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Organization>>;
}
