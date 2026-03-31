use crate::domain::errors::{DomainError, Result};
use regex::Regex;

/// Represents an email address as a value object in the domain.
/// Value objects are immutable and compared by value, not identity.
#[derive(Debug, Clone, PartialEq)]
pub struct Email {
    pub value: String,
}

impl Email {
    /// Creates a new Email value object with validation.
    /// Validates the email format using a regex pattern.
    pub fn new(value: String) -> Result<Self> {
        // RFC 5322 compliant email regex (requires at least one dot in domain)
        let email_regex = Regex::new(
            r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+$"
        ).map_err(|_| DomainError::custom("Invalid regex pattern"))?;

        if email_regex.is_match(&value) {
            Ok(Self { value })
        } else {
            Err(DomainError::InvalidEmail(format!(
                "Invalid email: {}",
                value
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::DomainError;

    #[test]
    fn test_email_creation_valid() {
        let email = Email::new("test@example.com".to_string());
        assert!(email.is_ok());
        assert_eq!(email.unwrap().value, "test@example.com");
    }

    #[test]
    fn test_email_creation_invalid() {
        let email = Email::new("invalid-email".to_string());
        assert!(email.is_err());
        assert!(matches!(
            email.unwrap_err(),
            DomainError::InvalidEmail(msg) if msg == "Invalid email: invalid-email"
        ));
    }

    #[test]
    fn test_email_creation_edge_cases() {
        // Valid cases
        assert!(Email::new("user@domain.com".to_string()).is_ok());
        assert!(Email::new("user.name+tag@domain.co.uk".to_string()).is_ok());

        // Invalid cases
        assert!(Email::new("user@".to_string()).is_err());
        assert!(Email::new("@domain.com".to_string()).is_err());
        assert!(Email::new("user".to_string()).is_err());
        assert!(Email::new("user@domain".to_string()).is_err());
        assert!(Email::new("user@@domain.com".to_string()).is_err());
        assert!(Email::new("user@localhost".to_string()).is_err()); // localhost not allowed for domain emails
    }

    #[test]
    fn test_email_equality() {
        let email1 = Email::new("test@example.com".to_string()).unwrap();
        let email2 = Email::new("test@example.com".to_string()).unwrap();
        let email3 = Email::new("other@example.com".to_string()).unwrap();

        assert_eq!(email1, email2);
        assert_ne!(email1, email3);
    }

    #[test]
    fn test_email_clone() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let cloned = email.clone();
        assert_eq!(email, cloned);
    }
}
