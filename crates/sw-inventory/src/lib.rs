use std::collections::HashMap;
use sw_errors::{Result, WeaveError};
use sw_types::{Part, Shortage};
pub fn project(
    requirements: &HashMap<String, u32>,
    parts: &HashMap<String, Part>,
    severity: f64,
) -> Result<Vec<Shortage>> {
    if !(0.0..=1.0).contains(&severity) {
        return Err(WeaveError::Invalid("severity range".into()));
    }
    let mut out = vec![];
    for (id, required) in requirements {
        let p = parts
            .get(id)
            .ok_or_else(|| WeaveError::Invalid(format!("unknown part {id}")))?;
        let usable = ((p.on_hand.saturating_sub(p.safety_stock)) as f64 * (1.0 - severity * 0.15))
            .floor() as u32;
        let gap = required.saturating_sub(usable);
        if gap > 0 {
            out.push(Shortage {
                part_id: id.clone(),
                tier: p.tier,
                required: *required,
                available: usable,
                gap,
            });
        }
    }
    out.sort_by_key(|s| (s.tier, std::cmp::Reverse(s.gap)));
    Ok(out)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn safety_stock_is_not_consumed() {
        let mut p = HashMap::new();
        p.insert(
            "a".into(),
            Part {
                id: "a".into(),
                name: "A".into(),
                tier: 1,
                on_hand: 10,
                safety_stock: 4,
                suppliers: vec![],
            },
        );
        let r = HashMap::from([("a".into(), 8)]);
        assert_eq!(project(&r, &p, 0.0).unwrap()[0].gap, 2);
    }
}
