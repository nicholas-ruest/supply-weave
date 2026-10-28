use std::{path::PathBuf, sync::Mutex, time::Instant};
use sw_errors::{Result, WeaveError};
use sw_repository::Repository;
use sw_ruvector::PlanIndex;
use sw_supply_network::Network;
use sw_types::{CreatePlan, Disruption, Plan};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExecutionMode {
    Combined,
    RuvnetOnly,
    EnterpriseOnly,
    NoRuvector,
    NoRvf,
    NoDatafusion,
    NoDro,
}

impl ExecutionMode {
    fn from_env() -> Self {
        match std::env::var("SUPPLY_WEAVE_ABLATION").as_deref() {
            Ok("ruvnet-only") => Self::RuvnetOnly,
            Ok("enterprise-only") => Self::EnterpriseOnly,
            Ok("no-ruvector") => Self::NoRuvector,
            Ok("no-rvf") => Self::NoRvf,
            Ok("no-datafusion") => Self::NoDatafusion,
            Ok("no-dro") => Self::NoDro,
            _ => Self::Combined,
        }
    }

    fn uses_datafusion(self) -> bool {
        !matches!(self, Self::RuvnetOnly | Self::NoDatafusion)
    }

    fn uses_ruvector(self) -> bool {
        !matches!(self, Self::EnterpriseOnly | Self::NoRuvector)
    }

    fn uses_rvf(self) -> bool {
        !matches!(self, Self::EnterpriseOnly | Self::NoRvf)
    }

    fn uses_dro(self) -> bool {
        self != Self::NoDro
    }
}

pub struct Orchestrator {
    repo: Repository,
    index: Mutex<PlanIndex>,
    network: Network,
    mode: ExecutionMode,
}

impl Orchestrator {
    pub fn open(path: PathBuf) -> Result<Self> {
        let mode = ExecutionMode::from_env();
        let repo = Repository::open(path)?;
        let mut index = PlanIndex::new()?;
        if mode.uses_ruvector() {
            for plan in repo.list()? {
                index.insert(&plan.id, sw_embedding::embed(&plan.disruption))?;
            }
        }
        Ok(Self {
            repo,
            index: Mutex::new(index),
            network: Network::build(sw_seed::suppliers(), sw_seed::parts())?,
            mode,
        })
    }

    pub async fn create(
        &self,
        req: CreatePlan,
        correlation_id: String,
        idempotency: String,
    ) -> Result<Plan> {
        if let Some(plan) = self
            .repo
            .list()?
            .into_iter()
            .find(|plan| plan.correlation_id == idempotency)
        {
            return Ok(plan);
        }
        if req.region.trim().is_empty()
            || req.demand_units == 0
            || !(0.0..=1.0).contains(&req.severity)
        {
            return Err(WeaveError::Invalid("request bounds".into()));
        }

        let started = Instant::now();
        let disruption = Disruption {
            id: req.disruption_id,
            region: req.region,
            severity: req.severity,
            duration_days: req.duration_days,
            demand_units: req.demand_units,
            require_certified: req.require_certified,
        };
        let mut plan = Plan::new(idempotency, disruption.clone());
        plan.audit
            .push(sw_audit::event("scenario", &correlation_id));
        plan.audit
            .push(sw_audit::event("mode", &format!("{:?}", self.mode)));

        let requirements = sw_bom::explode("edge-unit", disruption.demand_units, &sw_seed::bom())?;
        plan.shortages =
            sw_inventory::project(&requirements, &self.network.parts, disruption.severity)?;
        plan.audit.push(sw_audit::event(
            "shortage",
            &plan.shortages.len().to_string(),
        ));

        let effective_spend = if self.mode.uses_datafusion() {
            let (capacity, average_cost) = sw_datafusion::regional_capacity(
                &self.network.suppliers.values().cloned().collect::<Vec<_>>(),
                &disruption.region,
            )
            .await?;
            plan.audit.push(sw_audit::event(
                "query",
                &format!("DataFusion capacity={capacity} avg={average_cost:.2}"),
            ));
            req.spend_limit
        } else {
            plan.audit
                .push(sw_audit::event("query", "DataFusion removed"));
            req.spend_limit * 0.85
        };

        if self.mode.uses_ruvector() {
            plan.analogues = {
                let index = self
                    .index
                    .lock()
                    .map_err(|_| WeaveError::Dependency("index lock".into()))?;
                sw_recall::analogous(&index, &disruption)?
            };
            plan.audit.push(sw_audit::event(
                "recall",
                &format!("RuVector hits={}", plan.analogues.len()),
            ));
        } else {
            plan.audit
                .push(sw_audit::event("recall", "RuVector removed"));
        }

        let mut candidates = if self.mode.uses_dro() {
            sw_substitution::rank(
                &plan.shortages,
                &self.network,
                disruption.require_certified,
                disruption.severity,
            )?
        } else {
            sw_substitution::rank_naive(
                &plan.shortages,
                &self.network,
                disruption.require_certified,
                disruption.severity,
            )?
        };
        let recall_bonus = sw_recall::stability_bonus(&plan.analogues);
        for candidate in &mut candidates {
            candidate.score += recall_bonus;
        }
        plan.candidates = candidates;
        plan.waves = sw_wave::schedule(&plan.candidates, &self.network.suppliers, effective_spend)?;
        let optimizer = if self.mode.uses_dro() { "DRO" } else { "naive" };
        plan.audit.push(sw_audit::event(
            "optimize",
            &format!("{optimizer} waves={}", plan.waves.len()),
        ));

        sw_policy::authorize(
            &plan.waves,
            &self.network.suppliers,
            disruption.require_certified,
            req.max_concentration,
        )?;
        plan.audit.push(sw_audit::event("policy", "authorized"));
        plan.metrics = sw_evaluation::measure(
            &plan.shortages,
            &plan.waves,
            started.elapsed().as_millis() as u64,
        );
        plan.status = "authorized".into();

        if self.mode.uses_rvf() {
            plan.audit.push(sw_audit::event("persist", "RVF append"));
            self.repo.save(&plan)?;
        } else {
            plan.audit.push(sw_audit::event("persist", "RVF removed"));
        }
        if self.mode.uses_ruvector() {
            self.index
                .lock()
                .map_err(|_| WeaveError::Dependency("index lock".into()))?
                .insert(&plan.id, sw_embedding::embed(&disruption))?;
        }
        Ok(plan)
    }

    pub fn list(&self) -> Result<Vec<Plan>> {
        self.repo.list()
    }

    pub fn get(&self, id: &str) -> Result<Plan> {
        self.repo.get(id)
    }

    pub fn count(&self) -> Result<usize> {
        self.repo.count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn denial_has_no_durable_effect() {
        let directory = tempfile::tempdir().unwrap();
        let orchestrator = Orchestrator::open(directory.path().into()).unwrap();
        let request = CreatePlan {
            disruption_id: "x".into(),
            region: "NA".into(),
            severity: 0.7,
            duration_days: 20,
            demand_units: 700,
            require_certified: true,
            max_concentration: 0.2,
            spend_limit: 1.0,
        };
        assert!(orchestrator
            .create(request, "corr-123456".into(), "idem-123456".into())
            .await
            .is_err());
        assert_eq!(orchestrator.count().unwrap(), 0);
    }
}
