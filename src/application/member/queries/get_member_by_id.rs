use crate::domain::entities::member::Member;
use crate::domain::errors::DomainError;
use crate::domain::repository::member_repository::MemberRepository;
use uuid::Uuid;

pub struct GetMemberByIdQuery {
    pub id: Uuid,
}

pub async fn handle_get_member_by_id(
    repo: &dyn MemberRepository,
    query: GetMemberByIdQuery,
) -> Result<Option<Member>, DomainError> {
    repo.find_by_id(query.id).await
}
