use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::errors::Result;
use crate::domain::{Email, MemberRole};

#[derive(Debug, Clone)]
pub struct Member {
    id: Uuid,                  // Identity - never changes
    organization_id: Uuid,     // Foreign key to another Aggregate (Organization)
    division_id: Option<Uuid>, // Optional reference to Division Aggregate

    // Value Objects for rich domain behavior
    name: String,
    email: Option<Email>,
    role: MemberRole,

    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl Member {
    // Factory method - enforces invariants
    pub fn new(
        organization_id: Uuid,
        name: String,
        email: Option<String>,
        role: MemberRole,
        division_id: Option<Uuid>,
    ) -> Result<Self> {
        let now = OffsetDateTime::now_utc();
        let name = name;
        let email = match email {
            Some(e) => Some(Email::new(e)?),
            None => None,
        };

        Ok(Member {
            id: Uuid::new_v4(),
            organization_id,
            division_id,
            name,
            email,
            role,
            created_at: now,
            updated_at: now,
        })
    }

    // Getters (immutable access)
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn organization_id(&self) -> Uuid {
        self.organization_id
    }
    pub fn division_id(&self) -> Option<Uuid> {
        self.division_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn email(&self) -> Option<&str> {
        self.email.as_ref().map(|e| e.value.as_str())
    }
    pub fn role(&self) -> MemberRole {
        self.role
    }

    // Behavior - business methods
    pub fn change_name(&mut self, new_name: String) -> Result<()> {
        self.name = new_name;
        self.updated_at = OffsetDateTime::now_utc();
        Ok(())
    }

    pub fn change_role(&mut self, new_role: MemberRole) {
        self.role = new_role;
        self.updated_at = OffsetDateTime::now_utc();
    }

    pub fn assign_to_division(&mut self, division_id: Uuid) {
        self.division_id = Some(division_id);
        self.updated_at = OffsetDateTime::now_utc();
    }

    pub fn remove_from_division(&mut self) {
        self.division_id = None;
        self.updated_at = OffsetDateTime::now_utc();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::DomainError;

    #[test]
    fn test_member_creation_valid_with_email_and_division() {
        let organization_id = Uuid::new_v4();
        let division_id = Uuid::new_v4();
        let member = Member::new(
            organization_id,
            "Alice".to_string(),
            Some("alice@example.com".to_string()),
            MemberRole::Manager,
            Some(division_id),
        )
        .unwrap();

        assert_eq!(member.organization_id(), organization_id);
        assert_eq!(member.division_id(), Some(division_id));
        assert_eq!(member.name(), "Alice");
        assert_eq!(member.email(), Some("alice@example.com"));
        assert_eq!(member.role(), MemberRole::Manager);
    }

    #[test]
    fn test_member_creation_valid_without_optional_fields() {
        let organization_id = Uuid::new_v4();
        let member = Member::new(
            organization_id,
            "Bob".to_string(),
            None,
            MemberRole::Member,
            None,
        )
        .unwrap();

        assert_eq!(member.organization_id(), organization_id);
        assert_eq!(member.division_id(), None);
        assert_eq!(member.name(), "Bob");
        assert_eq!(member.email(), None);
        assert_eq!(member.role(), MemberRole::Member);
    }

    #[test]
    fn test_member_creation_invalid_email_returns_error() {
        let result = Member::new(
            Uuid::new_v4(),
            "Chris".to_string(),
            Some("invalid-email".to_string()),
            MemberRole::Guest,
            None,
        );

        assert!(matches!(
            result,
            Err(DomainError::InvalidEmail(msg)) if msg == "Invalid email: invalid-email"
        ));
    }

    #[test]
    fn test_member_mutation_methods() {
        let mut member = Member::new(
            Uuid::new_v4(),
            "Dana".to_string(),
            Some("dana@example.com".to_string()),
            MemberRole::Member,
            None,
        )
        .unwrap();

        member.change_name("Dana Updated".to_string()).unwrap();
        assert_eq!(member.name(), "Dana Updated");

        member.change_role(MemberRole::Admin);
        assert_eq!(member.role(), MemberRole::Admin);

        let division_id = Uuid::new_v4();
        member.assign_to_division(division_id);
        assert_eq!(member.division_id(), Some(division_id));

        member.remove_from_division();
        assert_eq!(member.division_id(), None);
    }
}
