# Architecture

The first runnable slice is browser planner -> versioned API -> auth, validation, correlation and idempotency -> BOM/inventory -> DataFusion SQL and RuVector recall -> robust substitution/waves -> policy -> RVF append -> rendered plan. Policy denial returns before repository save. Startup reads RVF and reconstructs RuVector.

## Crate map
Every counted crate has a typed public API, a real caller or executable role, focused tests and decision/context links.

|#|Crate|Responsibility|Callers|Acceptance/failure|Links|
|---:|---|---|---|---|---|
|1|`sw-types`|shared aggregates|all crates|serialization|ADR-0001 / DDD-01|
|2|`sw-errors`|error contracts|all fallible crates|typed mapping|ADR-0002 / DDD-02|
|3|`sw-config`|runtime bounds|API|invalid config|ADR-0003 / DDD-03|
|4|`sw-supply-network`|supplier graph|orchestrator/substitution|dangling references|ADR-0004 / DDD-04|
|5|`sw-bom`|BOM explosion|orchestrator|tier propagation|ADR-0005 / DDD-05|
|6|`sw-inventory`|shortage projection|orchestrator|safety stock|ADR-0006 / DDD-06|
|7|`sw-forecast`|conformal interval|scenario/evaluation|coverage|ADR-0007 / DDD-07|
|8|`sw-scenario`|disruption curve|orchestrator/embedding|monotonic recovery|ADR-0008 / DDD-08|
|9|`sw-datafusion`|scenario SQL|orchestrator|RecordBatch SQL|ADR-0009 / DDD-09|
|10|`sw-embedding`|disruption vectors|RuVector|determinism|ADR-0010 / DDD-10|
|11|`sw-ruvector`|HNSW insert/search|recall|actual search|ADR-0011 / DDD-11|
|12|`sw-recall`|analogue signal|orchestrator|bounded bonus|ADR-0012 / DDD-12|
|13|`sw-robust`|CVaR/ambiguity|substitution/evaluation|tail dominance|ADR-0013 / DDD-01|
|14|`sw-substitution`|supplier ranking|orchestrator|no-eligible denial|ADR-0014 / DDD-02|
|15|`sw-wave`|procurement schedule|orchestrator|spend bound|ADR-0015 / DDD-03|
|16|`sw-policy`|certification/concentration|orchestrator|no-effect denial|ADR-0016 / DDD-04|
|17|`sw-audit`|ordered evidence|orchestrator|sequence|ADR-0017 / DDD-05|
|18|`sw-rvf`|RVF append/reopen|repository|round trip|ADR-0018 / DDD-06|
|19|`sw-repository`|durable plans|orchestrator|save/get/list|ADR-0019 / DDD-07|
|20|`sw-auth`|principal/roles|middleware/API|viewer denial|ADR-0020 / DDD-08|
|21|`sw-middleware`|correlation/idempotency|API|malformed headers|ADR-0021 / DDD-09|
|22|`sw-seed`|synthetic BOM|orchestrator/E2E|multi-tier seed|ADR-0022 / DDD-10|
|23|`sw-evaluation`|metrics/ablations|orchestrator|baseline|ADR-0023 / DDD-11|
|24|`sw-orchestrator`|transactional use case|API|denial count zero|ADR-0024 / DDD-12|
|25|`sw-api`|Axum + React serving|browser|health/routes|ADR-0025 / DDD-01|

## Boundaries
DataFusion receives Arrow arrays and executes SQL. RuVector stores and searches 32-dimensional disruption embeddings; recall affects ranking but never authority. RVF is the only plan store. The allocator computes explicit tail loss. One container serves React and APIs with health/readiness and Cloud Run-compatible stateless configuration.

## Failure containment
Input, role and idempotency are checked before planning. Certification, concentration and spend failures occur before RVF append. Dependency and persistence faults use explicit errors. HTTP timeout bounds the composed request; no hidden mutation retry exists.
