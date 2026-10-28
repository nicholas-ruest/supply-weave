use sw_errors::{Result, WeaveError};
pub fn conformal_interval(history: &[u32], point: u32, alpha: f64) -> Result<(u32, u32)> {
    if history.len() < 4 || !(0.0..0.5).contains(&alpha) {
        return Err(WeaveError::Invalid("forecast sample/alpha".into()));
    }
    let mut residuals: Vec<u32> = history.iter().map(|x| x.abs_diff(point)).collect();
    residuals.sort_unstable();
    let i = (((1.0 - alpha) * (residuals.len() + 1) as f64).ceil() as usize)
        .saturating_sub(1)
        .min(residuals.len() - 1);
    let q = residuals[i];
    Ok((point.saturating_sub(q), point.saturating_add(q)))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interval_covers_point() {
        let x = conformal_interval(&[90, 95, 105, 110], 100, 0.2).unwrap();
        assert!(x.0 <= 100 && x.1 >= 100);
    }
}
