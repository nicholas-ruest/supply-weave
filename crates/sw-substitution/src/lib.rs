use sw_errors::{Result, WeaveError};
use sw_supply_network::Network;
use sw_types::{Candidate, Shortage};

pub fn rank(
    shortages: &[Shortage],
    network: &Network,
    certified: bool,
    severity: f64,
) -> Result<Vec<Candidate>> {
    rank_with(shortages, network, certified, severity, true)
}

pub fn rank_naive(
    shortages: &[Shortage],
    network: &Network,
    certified: bool,
    severity: f64,
) -> Result<Vec<Candidate>> {
    rank_with(shortages, network, certified, severity, false)
}

fn rank_with(
    shortages: &[Shortage],
    network: &Network,
    certified: bool,
    severity: f64,
    robust: bool,
) -> Result<Vec<Candidate>> {
    let mut output = vec![];
    for gap in shortages {
        let suppliers = network.eligible(&gap.part_id, certified);
        if suppliers.is_empty() {
            return Err(WeaveError::Denied(format!(
                "no eligible supplier for {}",
                gap.part_id
            )));
        }
        for supplier in suppliers {
            let quantity = gap
                .gap
                .min(((supplier.capacity as f64) * (1.0 - severity * 0.5)) as u32);
            let losses = [
                supplier.unit_cost * (1.0 + severity),
                f64::from(supplier.lead_days) * severity,
                f64::from(gap.gap.saturating_sub(quantity)) * 4.0,
            ];
            let tail_loss = if robust {
                sw_robust::cvar(&losses, 0.67)?
            } else {
                0.0
            };
            let score = f64::from(quantity)
                / (1.0 + supplier.unit_cost + f64::from(supplier.lead_days) * 0.2 + tail_loss);
            output.push(Candidate {
                part_id: gap.part_id.clone(),
                supplier_id: supplier.id.clone(),
                quantity,
                score,
                tail_loss,
                rationale: if robust {
                    format!(
                        "{} units; {}d lead; CVaR {:.2}",
                        quantity, supplier.lead_days, tail_loss
                    )
                } else {
                    format!(
                        "{} units; {}d lead; naive ranking",
                        quantity, supplier.lead_days
                    )
                },
            });
        }
    }
    output.sort_by(|left, right| right.score.total_cmp(&left.score));
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_network_denies() {
        let network = Network {
            suppliers: Default::default(),
            parts: Default::default(),
        };
        let shortage = Shortage {
            part_id: "x".into(),
            tier: 1,
            required: 1,
            available: 0,
            gap: 1,
        };
        assert!(rank(&[shortage], &network, true, 0.5).is_err());
    }

    #[test]
    fn naive_mode_removes_tail_risk_mechanism() {
        let network = Network::build(sw_seed::suppliers(), sw_seed::parts()).unwrap();
        let shortages = [Shortage {
            part_id: "controller".into(),
            tier: 1,
            required: 100,
            available: 0,
            gap: 100,
        }];
        assert!(rank_naive(&shortages, &network, true, 0.5)
            .unwrap()
            .iter()
            .all(|candidate| candidate.tail_loss == 0.0));
    }
}
