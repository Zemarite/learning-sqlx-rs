use crate::domain::errors::{DomainError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemberRole {
    Admin,
    Manager,
    Member,
    Guest,
    // ... extend based on business rules
}

impl MemberRole {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(MemberRole::Admin),
            "manager" => Ok(MemberRole::Manager),
            "member" => Ok(MemberRole::Member),
            "guest" => Ok(MemberRole::Guest),
            _ => Err(DomainError::InvalidRole(s.to_string())),
        }
    }
}
