use sw_auth::Principal;
use sw_errors::{Result, WeaveError};
pub fn correlation(value: Option<&str>) -> Result<String> {
    let v = value
        .map(str::to_string)
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    if v.len() < 8
        || v.len() > 128
        || !v
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_".contains(c))
    {
        return Err(WeaveError::Invalid("correlation id".into()));
    }
    Ok(v)
}
pub fn idempotency(value: Option<&str>) -> Result<String> {
    let v = value.ok_or_else(|| WeaveError::Invalid("idempotency key required".into()))?;
    if v.len() < 8 || v.len() > 80 {
        return Err(WeaveError::Invalid("idempotency key length".into()));
    }
    Ok(v.to_string())
}
pub fn authorize_plan(p: &Principal) -> Result<()> {
    sw_auth::require(p, "planner")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn short_key_fails() {
        assert!(idempotency(Some("x")).is_err());
    }
}
