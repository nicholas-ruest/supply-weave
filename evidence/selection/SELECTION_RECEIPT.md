# SELECTION_RECEIPT - 2026-10-27 - Supply Weave
Gate: PASS. Full-stack multi-tier supply-disruption planning and recovery, not a CLI, validator, wrapper, benchmark, or one-feature service.

Capability: turn a multi-tier BOM, supplier capacities, inventory, and disruption into a policy-safe, risk-aware substitution and procurement-wave plan that operators save, audit, recover, and reopen after restart.

Distinctness: recent builds govern agent continuity, context, authorization, delivery, incidents, SLOs, data purpose, inference routing, authority, and API evolution. Slot Sweep was a narrow reservation analyzer. Supply Weave operates procurement: BOM propagation, qualification, inventory depletion, lead-time uncertainty, substitution, and staged recovery. Its artifact is a sourcing plan, not a policy verdict, capacity audit, failover route, or contract decision.

Users/workload: procurement resilience, manufacturing, supplier quality, and operations teams model a port closure or supplier outage against an electronics BOM, analyze production impact, create and correct a plan, save it, restart, and retrieve the audit.

Bounded contexts:
1. Supply Network: suppliers, parts, facilities, certifications, capacity, lead time.
2. Demand/Inventory: BOM explosion, demand windows, stock, shortage projection.
3. Scenario Intelligence: DataFusion scenario queries and RuVector analogous-plan recall.
4. Robust Allocation: ambiguity weights, CVaR substitute selection, procurement waves.
5. Governance: approved source, certification, concentration, spend, idempotency, no-effect denial.
6. Evidence/Recovery: RVF plan/audit persistence, correlation, restart retrieval, evaluation.

Ingredients:
- ruvector-core 2.4.0 at RuVector 4f6bbf9086e97fb6d1560e211aa740e5e7a75917: real nearest-neighbour recall.
- rvf-runtime 0.4.0 at same revision: append-only plan/audit store.
- Apache DataFusion 55.2.0 at c1013ec583b46d55323d8fb173a064c58fa15d2b, Apache-2.0: real SQL/DataFrame execution.
- Frontier: two-stage risk-averse DRO with CVaR tail penalty and ambiguity reweighting, grounded in arXiv:2601.14665 and 2604.23140; changes scenario weights, substitutes, safety capacity, and waves.

Success: primary renders affected tiers, shortage, substitutes, cost, coverage, risk, orders after all integrations. Denial rejects uncertified/over-concentrated sourcing with zero RVF append, then accepts correction. Durable retrieval reopens plan/audit after refresh and restart. P95 under 1.5s; coverage >=95%; denial append delta zero.

Ablations: combined; Ruvnet-only; enterprise-only; remove RuVector, RVF, DataFusion, robust mechanism. Expected degradation: analog stability, durable audit, scenario quality, worst-case shortage.

Preflight: Rust 1.99.1, Node 20.20.2, npm 10.8.2, Playwright 1.64.1, GitHub, browser, hosted CI. Local Docker absent; hosted CI container verification mandatory.

CORE_MEMORY_RECEIPT: 1807822e55f5216d0a5203fe8c7c4834234b93e3. Required guidance/checker hashes captured separately.
LESSON_APPLICATION_MAP: VERIFIED_APPROVED=NONE_AVAILABLE; held=[]; unscreened empty task_start not credited.
