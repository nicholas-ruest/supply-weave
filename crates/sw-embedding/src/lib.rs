use sha2::{Digest, Sha256};
use sw_types::Disruption;
pub const DIMENSION: usize = 32;
pub fn embed(d: &Disruption) -> Vec<f32> {
    let mut h = Sha256::new();
    h.update(d.id.as_bytes());
    h.update(d.region.as_bytes());
    h.update(d.severity.to_le_bytes());
    h.update(d.duration_days.to_le_bytes());
    h.update(d.demand_units.to_le_bytes());
    h.finalize().iter().map(|b| f32::from(*b) / 255.0).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic_dimension() {
        let d = Disruption {
            id: "d".into(),
            region: "EU".into(),
            severity: 0.2,
            duration_days: 2,
            demand_units: 4,
            require_certified: true,
        };
        assert_eq!(embed(&d).len(), DIMENSION);
        assert_eq!(embed(&d), embed(&d));
    }
}
