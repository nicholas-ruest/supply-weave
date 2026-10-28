use std::collections::{HashMap, HashSet, VecDeque};
use sw_errors::{Result, WeaveError};
use sw_types::BomEdge;
pub fn explode(root: &str, units: u32, edges: &[BomEdge]) -> Result<HashMap<String, u32>> {
    if units == 0 {
        return Err(WeaveError::Invalid("demand must be positive".into()));
    }
    let mut out = HashMap::new();
    let mut q = VecDeque::from([(root.to_string(), units)]);
    let mut steps = 0;
    while let Some((parent, n)) = q.pop_front() {
        steps += 1;
        if steps > 1000 {
            return Err(WeaveError::Invalid("BOM cycle or explosion".into()));
        }
        let children: Vec<_> = edges.iter().filter(|e| e.parent == parent).collect();
        for e in children {
            let req = n
                .checked_mul(e.quantity)
                .ok_or_else(|| WeaveError::Invalid("BOM overflow".into()))?;
            *out.entry(e.child.clone()).or_default() += req;
            q.push_back((e.child.clone(), req));
        }
    }
    Ok(out)
}
pub fn acyclic(edges: &[BomEdge]) -> bool {
    let mut seen = HashSet::new();
    edges
        .iter()
        .all(|e| e.parent != e.child && seen.insert((e.parent.clone(), e.child.clone())))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn propagates_tiers() {
        let e = vec![
            BomEdge {
                parent: "x".into(),
                child: "a".into(),
                quantity: 2,
            },
            BomEdge {
                parent: "a".into(),
                child: "b".into(),
                quantity: 3,
            },
        ];
        assert_eq!(explode("x", 4, &e).unwrap()["b"], 24);
    }
}
