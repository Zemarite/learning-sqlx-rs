use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::value_object::OrganizationId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    id: OrganizationId,
    name: String,
    description: Option<String>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}
impl Organization {
    // Factory method - preferred way to create new entities in DDD
    pub fn new(name: String, description: Option<String>) -> Self {
        let now = OffsetDateTime::now_utc();

        Self {
            id: OrganizationId::new(),
            name: Self::validate_name(&name),
            description,
            created_at: now,
            updated_at: now,
        }
    }

    // Reconstruct from persistence (used by Repository)
    pub fn from_persistence(
        id: Uuid,
        name: String,
        description: Option<String>,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Self {
        Self {
            id: OrganizationId::from_uuid(id),
            name: Self::validate_name(&name),
            description,
            created_at,
            updated_at,
        }
    }

    // Business behavior / methods
    pub fn rename(&mut self, new_name: String) {
        self.name = Self::validate_name(&new_name);
        self.updated_at = OffsetDateTime::now_utc();
    }

    pub fn update_description(&mut self, new_description: Option<String>) {
        self.description = new_description;
        self.updated_at = OffsetDateTime::now_utc();
    }

    // Invariants / Validation
    fn validate_name(name: &str) -> String {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            panic!("Organization name cannot be empty"); // In real code, return Result
        }
        if trimmed.len() > 255 {
            panic!("Organization name is too long"); // Use proper error handling
        }
        trimmed.to_string()
    }

    // Getters (immutable access)
    pub fn id(&self) -> &OrganizationId {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }
    pub fn updated_at(&self) -> OffsetDateTime {
        self.updated_at
    }
}

// Make it easy to compare organizations by identity (important in DDD)
impl PartialEq for Organization {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Organization {}
