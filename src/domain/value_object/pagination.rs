use crate::domain::errors::DomainError;

/// Value Object for pagination, ensuring domain invariants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pagination {
    page: u32,     // 1-based page number
    per_page: u32, // Items per page (max 100 for performance)
}

impl Pagination {
    /// Factory method to create a valid Pagination, enforcing invariants.
    pub fn new(page: u32, per_page: u32) -> Result<Self, DomainError> {
        if page < 1 {
            return Err(DomainError::ValidationError(
                "Page must be at least 1".to_string(),
            ));
        }
        if per_page < 1 || per_page > 100 {
            return Err(DomainError::ValidationError(
                "Per page must be between 1 and 100".to_string(),
            ));
        }
        Ok(Self { page, per_page })
    }

    /// Calculates the database offset (0-based).
    pub fn offset(&self) -> i64 {
        ((self.page - 1) * self.per_page) as i64
    }

    /// Returns the limit for queries.
    pub fn limit(&self) -> i64 {
        self.per_page as i64
    }

    // Getters for read access (DDD discourages direct field access, but useful for queries)
    pub fn page(&self) -> u32 {
        self.page
    }
    pub fn per_page(&self) -> u32 {
        self.per_page
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_pagination() {
        let p = Pagination::new(1, 10).unwrap();
        assert_eq!(p.offset(), 0);
        assert_eq!(p.limit(), 10);
    }

    #[test]
    fn test_invalid_page() {
        assert!(Pagination::new(0, 10).is_err());
    }

    #[test]
    fn test_invalid_per_page() {
        assert!(Pagination::new(1, 0).is_err());
        assert!(Pagination::new(1, 101).is_err());
    }
}
