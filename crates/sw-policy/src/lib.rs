use std::collections::HashMap;
use sw_errors::{Result, WeaveError};
use sw_types::{OrderWave, Supplier};
pub fn authorize(
    waves: &[OrderWave],
    suppliers: &HashMap<String, Supplier>,
    require_certified: bool,
    max_concentration: f64,
) -> Result<()> {
    if waves.is_empty() {
        return Err(WeaveError::Denied("empty sourcing plan".into()));
    }
    let total: u32 = waves.iter().map(|w| w.quantity).sum();
    let mut by = HashMap::<&str, u32>::new();
    for w in waves {
        let s = suppliers
            .get(&w.supplier_id)
            .ok_or_else(|| WeaveError::Invalid("supplier disappeared".into()))?;
        if require_certified && !s.certified {
            return Err(WeaveError::Denied(format!(
                "supplier {} is not certified",
                s.id
            )));
        }
        *by.entry(&s.id).or_default() += w.quantity;
    }
    if by
        .values()
        .any(|n| f64::from(*n) / f64::from(total) > max_concentration)
    {
        return Err(WeaveError::Denied(
            "supplier concentration exceeds policy".into(),
        ));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_is_denied() {
        assert!(authorize(&[], &HashMap::new(), true, 0.8).is_err());
    }
}
