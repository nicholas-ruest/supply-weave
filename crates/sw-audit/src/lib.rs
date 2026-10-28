use sha2::{Digest, Sha256};
use sw_types::Plan;
pub fn event(action: &str, detail: &str) -> String {
    format!("{}|{}", action, detail)
}
pub fn plan_digest(p: &Plan) -> String {
    let bytes = serde_json::to_vec(p).expect("serializable plan");
    hex::encode(Sha256::digest(bytes))
}
pub fn has_sequence(events: &[String]) -> bool {
    [
        "scenario", "shortage", "query", "recall", "optimize", "policy", "persist",
    ]
    .iter()
    .all(|needle| events.iter().any(|e| e.starts_with(needle)))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn incomplete_sequence_fails() {
        assert!(!has_sequence(&[event("scenario", "x")]));
    }
}
