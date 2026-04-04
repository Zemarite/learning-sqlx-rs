use crate::domain::entities::organization::Organization;
use crate::domain::errors::DomainError;
use crate::domain::repository::organization_repository::OrganizationRepository;
use crate::domain::value_object::Pagination;

pub struct GetOrganizationsPaginatedQuery {
    pub pagination: Pagination,
}

pub async fn handle_get_organizations_paginated(
    repo: &dyn OrganizationRepository,
    query: GetOrganizationsPaginatedQuery,
) -> Result<Vec<Organization>, DomainError> {
    repo.find_paginated(&query.pagination).await
}