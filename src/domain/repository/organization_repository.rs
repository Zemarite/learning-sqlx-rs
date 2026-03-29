use crate::domain::entities::Organization;
use crate::domain::value_object::OrganizationId;

pub trait OrganizationRepository {
    async fn find_all(&self) -> Result<Vec<Organization>, RepositoryError>;
    async fn find_by_id(&self, id: OrganizationId)
    -> Result<Option<Organization>, RepositoryError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Organization>, RepositoryError>;
}
