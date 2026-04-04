use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, DomainError>;

#[derive(Debug, Display, From)]
#[display("{self:?}")]
pub enum DomainError {
    // -- enum error variants
    InvalidRole(String),

    // -- value object errors
    InvalidEmail(String),
    ValidationError(String),

    // Custom(String),
    #[from(String, &String, &str)]
    Custom(String),

    // -- Externals
    #[from]
    Io(std::io::Error), // as example
}

// region:    --- Custom

impl DomainError {
    pub fn custom_from_err(err: impl std::error::Error) -> Self {
        Self::Custom(err.to_string())
    }

    pub fn custom(val: impl Into<String>) -> Self {
        Self::Custom(val.into())
    }
}

// endregion: --- Custom

// region:    --- Error Boilerplate

impl std::error::Error for DomainError {}

// endregion: --- Error Boilerplate
