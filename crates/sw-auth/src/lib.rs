use sw_errors::{Result, WeaveError};
#[derive(Clone, Debug)]
pub struct Principal {
    pub subject: String,
    pub roles: Vec<String>,
}
pub fn authenticate(header: Option<&str>) -> Result<Principal> {
    let token = header
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| WeaveError::Denied("missing bearer token".into()))?;
    match token {
        "planner-token" => Ok(Principal {
            subject: "planner@seed".into(),
            roles: vec!["planner".into()],
        }),
        "viewer-token" => Ok(Principal {
            subject: "viewer@seed".into(),
            roles: vec!["viewer".into()],
        }),
        _ => Err(WeaveError::Denied("invalid token".into())),
    }
}
pub fn require(p: &Principal, role: &str) -> Result<()> {
    if p.roles.iter().any(|r| r == role) {
        Ok(())
    } else {
        Err(WeaveError::Denied(format!("role {role} required")))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn viewer_cannot_plan() {
        assert!(require(
            &authenticate(Some("Bearer viewer-token")).unwrap(),
            "planner"
        )
        .is_err());
    }
}
