use sw_errors::{Result, WeaveError};
use sw_types::Disruption;
pub fn daily_capacity_multiplier(d: &Disruption, day: u16) -> Result<f64> {
    if !(0.0..=1.0).contains(&d.severity) || d.duration_days == 0 {
        return Err(WeaveError::Invalid("disruption bounds".into()));
    }
    let active = day.min(d.duration_days) as f64 / d.duration_days as f64;
    Ok((1.0 - d.severity * (1.0 - active * 0.35)).clamp(0.05, 1.0))
}
pub fn scenario_vector(d: &Disruption) -> Vec<f32> {
    vec![
        d.severity as f32,
        d.duration_days as f32 / 90.0,
        d.demand_units as f32 / 10000.0,
        if d.require_certified { 1.0 } else { 0.0 },
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recovery_improves() {
        let d = Disruption {
            id: "x".into(),
            region: "r".into(),
            severity: 0.8,
            duration_days: 10,
            demand_units: 1,
            require_certified: true,
        };
        assert!(
            daily_capacity_multiplier(&d, 10).unwrap() > daily_capacity_multiplier(&d, 0).unwrap()
        );
    }
}
