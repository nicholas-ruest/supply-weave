use std::collections::{HashMap, HashSet};
use sw_errors::{Result, WeaveError};
use sw_types::{Part, Supplier};
#[derive(Clone)]
pub struct Network {
    pub suppliers: HashMap<String, Supplier>,
    pub parts: HashMap<String, Part>,
}
impl Network {
    pub fn build(suppliers: Vec<Supplier>, parts: Vec<Part>) -> Result<Self> {
        if suppliers.is_empty() || parts.is_empty() {
            return Err(WeaveError::Invalid(
                "network must contain suppliers and parts".into(),
            ));
        }
        let s: HashMap<_, _> = suppliers.into_iter().map(|x| (x.id.clone(), x)).collect();
        let mut ids = HashSet::new();
        for p in &parts {
            if !ids.insert(p.id.clone()) || p.suppliers.iter().any(|id| !s.contains_key(id)) {
                return Err(WeaveError::Invalid(format!("invalid part {}", p.id)));
            }
        }
        Ok(Self {
            suppliers: s,
            parts: parts.into_iter().map(|p| (p.id.clone(), p)).collect(),
        })
    }
    pub fn eligible(&self, part: &str, certified: bool) -> Vec<&Supplier> {
        self.parts
            .get(part)
            .map(|p| {
                p.suppliers
                    .iter()
                    .filter_map(|id| self.suppliers.get(id))
                    .filter(|s| !certified || s.certified)
                    .collect()
            })
            .unwrap_or_default()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_dangling_supplier() {
        let p = Part {
            id: "p".into(),
            name: "P".into(),
            tier: 1,
            on_hand: 0,
            safety_stock: 0,
            suppliers: vec!["missing".into()],
        };
        assert!(Network::build(vec![], vec![p]).is_err());
    }
}
