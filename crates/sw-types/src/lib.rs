use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Supplier {
    pub id: String,
    pub name: String,
    pub region: String,
    pub certified: bool,
    pub capacity: u32,
    pub lead_days: u16,
    pub unit_cost: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Part {
    pub id: String,
    pub name: String,
    pub tier: u8,
    pub on_hand: u32,
    pub safety_stock: u32,
    pub suppliers: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BomEdge {
    pub parent: String,
    pub child: String,
    pub quantity: u32,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Disruption {
    pub id: String,
    pub region: String,
    pub severity: f64,
    pub duration_days: u16,
    pub demand_units: u32,
    pub require_certified: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Shortage {
    pub part_id: String,
    pub tier: u8,
    pub required: u32,
    pub available: u32,
    pub gap: u32,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Candidate {
    pub part_id: String,
    pub supplier_id: String,
    pub quantity: u32,
    pub score: f64,
    pub tail_loss: f64,
    pub rationale: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OrderWave {
    pub wave: u8,
    pub day: u16,
    pub supplier_id: String,
    pub part_id: String,
    pub quantity: u32,
    pub cost: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Analogue {
    pub plan_id: String,
    pub distance: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Metrics {
    pub coverage: f64,
    pub total_cost: f64,
    pub cvar: f64,
    pub worst_shortage: u32,
    pub latency_ms: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Plan {
    pub id: String,
    pub created_at: String,
    pub correlation_id: String,
    pub disruption: Disruption,
    pub shortages: Vec<Shortage>,
    pub candidates: Vec<Candidate>,
    pub waves: Vec<OrderWave>,
    pub analogues: Vec<Analogue>,
    pub metrics: Metrics,
    pub status: String,
    pub audit: Vec<String>,
}
impl Plan {
    pub fn new(correlation_id: String, disruption: Disruption) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            correlation_id,
            disruption,
            shortages: vec![],
            candidates: vec![],
            waves: vec![],
            analogues: vec![],
            metrics: Metrics {
                coverage: 0.0,
                total_cost: 0.0,
                cvar: 0.0,
                worst_shortage: 0,
                latency_ms: 0,
            },
            status: "draft".into(),
            audit: vec![],
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePlan {
    pub disruption_id: String,
    pub region: String,
    pub severity: f64,
    pub duration_days: u16,
    pub demand_units: u32,
    pub require_certified: bool,
    pub max_concentration: f64,
    pub spend_limit: f64,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plan_identity() {
        let p = Plan::new(
            "c".into(),
            Disruption {
                id: "d".into(),
                region: "APAC".into(),
                severity: 0.5,
                duration_days: 7,
                demand_units: 10,
                require_certified: true,
            },
        );
        assert!(!p.id.is_empty());
    }
}
