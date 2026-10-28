use std::collections::HashSet;
use sw_errors::{Result, WeaveError};
use sw_types::{Candidate, OrderWave, Supplier};
pub fn schedule(
    candidates: &[Candidate],
    suppliers: &std::collections::HashMap<String, Supplier>,
    limit: f64,
) -> Result<Vec<OrderWave>> {
    let mut covered = HashSet::new();
    let mut out = vec![];
    let mut cost = 0.0;
    for c in candidates {
        if covered.contains(&c.part_id) || c.quantity == 0 {
            continue;
        }
        let s = suppliers
            .get(&c.supplier_id)
            .ok_or_else(|| WeaveError::Invalid("candidate supplier".into()))?;
        let next = f64::from(c.quantity) * s.unit_cost;
        if cost + next > limit {
            continue;
        }
        cost += next;
        covered.insert(c.part_id.clone());
        out.push(OrderWave {
            wave: if s.lead_days <= 10 {
                1
            } else if s.lead_days <= 20 {
                2
            } else {
                3
            },
            day: s.lead_days.saturating_sub(2),
            supplier_id: s.id.clone(),
            part_id: c.part_id.clone(),
            quantity: c.quantity,
            cost: next,
        });
    }
    if out.is_empty() {
        return Err(WeaveError::Denied("no plan within spend limit".into()));
    }
    out.sort_by_key(|x| (x.wave, x.day));
    Ok(out)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn spend_limit_blocks() {
        assert!(schedule(&[], &Default::default(), 1.0).is_err());
    }
}
