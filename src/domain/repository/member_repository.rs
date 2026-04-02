use crate::domain::Member;
use crate::domain::errors::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait MemberRepository {
    async fn save(&self, member: &Member) -> Result<()>;
    async fn find_all(&self) -> Result<Vec<Member>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Member>>;
    async fn find_by_organization(&self, org_id: Uuid) -> Result<Vec<Member>>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
