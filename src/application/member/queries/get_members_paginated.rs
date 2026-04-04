use crate::domain::entities::member::Member;
use crate::domain::errors::DomainError;
use crate::domain::repository::member_repository::MemberRepository;
use crate::domain::value_object::Pagination;

pub struct GetMembersPaginatedQuery {
    pub pagination: Pagination,
}

pub async fn handle_get_members_paginated(
    repo: &dyn MemberRepository,
    query: GetMembersPaginatedQuery,
) -> Result<Vec<Member>, DomainError> {
    repo.find_paginated(&query.pagination).await
}