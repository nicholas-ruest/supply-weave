use sw_types::{Metrics, OrderWave, Shortage};
pub fn measure(shortages: &[Shortage], waves: &[OrderWave], latency_ms: u64) -> Metrics {
    let required: u32 = shortages.iter().map(|s| s.gap).sum();
    let ordered: u32 = waves.iter().map(|w| w.quantity).sum();
    let losses: Vec<f64> = shortages.iter().map(|s| f64::from(s.gap)).collect();
    Metrics {
        coverage: if required == 0 {
            1.0
        } else {
            (f64::from(ordered) / f64::from(required)).min(1.0)
        },
        total_cost: waves.iter().map(|w| w.cost).sum(),
        cvar: sw_robust::cvar(&losses, 0.67).unwrap_or(0.0),
        worst_shortage: shortages.iter().map(|s| s.gap).max().unwrap_or(0),
        latency_ms,
    }
}
pub fn score(m: &Metrics) -> f64 {
    m.coverage * 100.0 - m.cvar * 0.05 - m.total_cost / 100000.0
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn perfect_empty_case() {
        assert_eq!(measure(&[], &[], 0).coverage, 1.0);
    }
}
