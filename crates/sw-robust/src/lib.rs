use sw_errors::{Result, WeaveError};
pub fn ambiguity_weights(losses: &[f64], radius: f64) -> Result<Vec<f64>> {
    if losses.is_empty() || radius < 0.0 {
        return Err(WeaveError::Invalid("DRO inputs".into()));
    }
    let max = losses.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let raw: Vec<_> = losses
        .iter()
        .map(|x| ((x - max) * radius.max(0.001)).exp())
        .collect();
    let z: f64 = raw.iter().sum();
    Ok(raw.into_iter().map(|x| x / z).collect())
}
pub fn cvar(losses: &[f64], alpha: f64) -> Result<f64> {
    if losses.is_empty() || !(0.5..1.0).contains(&alpha) {
        return Err(WeaveError::Invalid("CVaR inputs".into()));
    }
    let mut v = losses.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    let i = ((alpha * v.len() as f64).floor() as usize).min(v.len() - 1);
    Ok(v[i..].iter().sum::<f64>() / (v.len() - i) as f64)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tail_exceeds_mean() {
        let l = [1., 2., 9., 10.];
        assert!(cvar(&l, 0.75).unwrap() > l.iter().sum::<f64>() / 4.0);
    }
}
