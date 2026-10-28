use sw_errors::Result;
use sw_ruvector::PlanIndex;
use sw_types::{Analogue, Disruption};
pub fn analogous(index: &PlanIndex, d: &Disruption) -> Result<Vec<Analogue>> {
    index.search(&sw_embedding::embed(d), 3)
}
pub fn stability_bonus(a: &[Analogue]) -> f64 {
    a.iter()
        .map(|x| (1.0 / (1.0 + x.distance)).min(1.0))
        .sum::<f64>()
        * 0.03
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bonus_is_bounded() {
        assert!(
            stability_bonus(&[Analogue {
                plan_id: "p".into(),
                distance: 1.0
            }]) <= 0.03
        );
    }
}
