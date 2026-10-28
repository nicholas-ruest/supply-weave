use sw_types::{BomEdge, Part, Supplier};
pub fn suppliers() -> Vec<Supplier> {
    vec![
        Supplier {
            id: "pacifica-certified".into(),
            name: "Pacifica Circuits".into(),
            region: "APAC".into(),
            certified: true,
            capacity: 2400,
            lead_days: 14,
            unit_cost: 8.4,
        },
        Supplier {
            id: "nordic-qualified".into(),
            name: "Nordic Components".into(),
            region: "EU".into(),
            certified: true,
            capacity: 1600,
            lead_days: 9,
            unit_cost: 9.8,
        },
        Supplier {
            id: "rapid-uncertified".into(),
            name: "Rapid Parts".into(),
            region: "NA".into(),
            certified: false,
            capacity: 3200,
            lead_days: 5,
            unit_cost: 6.2,
        },
        Supplier {
            id: "atlas-certified".into(),
            name: "Atlas Semicon".into(),
            region: "NA".into(),
            certified: true,
            capacity: 1800,
            lead_days: 18,
            unit_cost: 7.9,
        },
    ]
}
pub fn parts() -> Vec<Part> {
    vec![
        Part {
            id: "controller".into(),
            name: "Controller".into(),
            tier: 1,
            on_hand: 220,
            safety_stock: 80,
            suppliers: vec!["pacifica-certified".into(), "nordic-qualified".into()],
        },
        Part {
            id: "sensor".into(),
            name: "Optical sensor".into(),
            tier: 2,
            on_hand: 900,
            safety_stock: 200,
            suppliers: vec!["rapid-uncertified".into(), "atlas-certified".into()],
        },
        Part {
            id: "substrate".into(),
            name: "Ceramic substrate".into(),
            tier: 3,
            on_hand: 1200,
            safety_stock: 300,
            suppliers: vec!["pacifica-certified".into(), "atlas-certified".into()],
        },
    ]
}
pub fn bom() -> Vec<BomEdge> {
    vec![
        BomEdge {
            parent: "edge-unit".into(),
            child: "controller".into(),
            quantity: 1,
        },
        BomEdge {
            parent: "edge-unit".into(),
            child: "sensor".into(),
            quantity: 2,
        },
        BomEdge {
            parent: "controller".into(),
            child: "substrate".into(),
            quantity: 2,
        },
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn realistic_seed() {
        assert!(suppliers().len() >= 4 && parts().iter().any(|p| p.tier == 3));
    }
}
