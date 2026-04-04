use crate::domain::entities::member::Member;
use crate::domain::errors::DomainError;
use crate::domain::repository::member_repository::MemberRepository;

pub async fn get_members(repo: &dyn MemberRepository) -> Result<Vec<Member>, DomainError> {
    repo.find_all().await
}
