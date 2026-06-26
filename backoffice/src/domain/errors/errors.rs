#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("invalid email address: {0}")]
    InvalidEmail(String),

    #[error("name cannot be empty")]
    EmptyName,

    #[error("name exceeds maximum length")]
    NameTooLong,

    #[error("name contains forbidden characters")]
    NameContainsForbiddenCharacters,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateUserError {
    #[error("a user with email {email} already exists")]
    Duplicate { email: Email },

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
