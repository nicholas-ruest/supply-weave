use thiserror::Error;
#[derive(Debug, Error)]
pub enum WeaveError {
    #[error("invalid input: {0}")]
    Invalid(String),
    #[error("policy denied: {0}")]
    Denied(String),
    #[error("dependency failed: {0}")]
    Dependency(String),
    #[error("persistence failed: {0}")]
    Persistence(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("timeout: {0}")]
    Timeout(String),
}
pub type Result<T> = std::result::Result<T, WeaveError>;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explicit_contract() {
        assert!(WeaveError::Denied("x".into())
            .to_string()
            .contains("denied"));
    }
}
