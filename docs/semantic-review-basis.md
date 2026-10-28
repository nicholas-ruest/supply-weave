# Semantic review basis

This review assesses behavior and linkage, not directory or line counts. The API executable calls authentication and middleware, then the orchestrator composes the remaining domain and infrastructure crates. Every row names the executed behavior and its verification point.

| Crate | Implemented behavior and real linkage | Failure or acceptance evidence |
|---|---|---|
| `sw-types` | Versioned request, plan, supplier, shortage, candidate, wave and metrics contracts consumed across the workspace and OpenAPI boundary. | Serialization/version contract test and API/client drift check. |
| `sw-errors` | Typed domain-to-HTTP error taxonomy used by every fallible domain and adapter boundary. | API error mapping exercised by validation and denial suites. |
| `sw-config` | Environment parsing and bounded runtime defaults used by `sw-api` and persistence startup. | Bounded-default test; clean launch/readiness. |
| `sw-supply-network` | Builds and validates the supplier/part qualification graph used by substitution. | Dangling supplier rejection test. |
| `sw-bom` | Detects cycles and recursively explodes multi-tier BOM demand for the orchestrator. | Tier propagation and cycle behavior test. |
| `sw-inventory` | Projects demand against on-hand, safety stock and lead-time supply. | Safety-stock non-consumption test. |
| `sw-forecast` | Computes a finite-sample conformal demand interval used by scenario planning. | Interval coverage test. |
| `sw-scenario` | Produces day-indexed disruption capacity curves and scenario vectors. | Invalid/monotone recovery behavior test. |
| `sw-datafusion` | Builds Arrow arrays, registers a `MemTable`, and executes DataFusion SQL for regional capacity and average lead time. | Real SQL test and live audit marker. |
| `sw-embedding` | Generates deterministic normalized 32-dimensional disruption embeddings consumed by RuVector. | Dimension/determinism test and HNSW search. |
| `sw-ruvector` | Owns real `ruvector-core` HNSW insert/search and maps results to saved plan identifiers. | Actual index-search test plus live analogue recall. |
| `sw-recall` | Converts HNSW neighbours into bounded analogue signals that alter ranking, never authority. | Stability-bound test and no-RuVector ablation. |
| `sw-robust` | Implements ambiguity weights and CVaR tail loss used by substitution ranking. | Tail-dominance test and no-DRO ablation. |
| `sw-substitution` | Ranks certified alternatives using capacity, lead time, cost, recall and robust tail loss; includes an explicit naive ablation path. | Empty-network denial and naive-tail-loss tests. |
| `sw-wave` | Schedules recovery purchase waves under lead-time and spend constraints. | Spend-limit failure test. |
| `sw-policy` | Applies certification, concentration and budget policy before any durable append. | Policy failure test and browser no-effect denial. |
| `sw-audit` | Constructs ordered evidence and plan digests consumed by persistence and UI. | Incomplete-sequence failure test. |
| `sw-rvf` | Uses `rvf-runtime` to append, reopen and read encoded plans from the only durable store. | Durable round-trip test and backend restart journey. |
| `sw-repository` | Provides save/get/list over RVF and reconstructs the RuVector index on startup. | Empty repository test; refresh/restart retrieval. |
| `sw-auth` | Parses bearer principals and enforces planner/viewer roles at the API boundary. | Viewer rejection test. |
| `sw-middleware` | Validates correlation and idempotency identifiers and calls authorization. | Missing/malformed header tests and idempotent live API path. |
| `sw-seed` | Supplies a coherent multi-tier synthetic BOM, suppliers and inventory through the real orchestrator interface. | Seed consistency test and browser journeys. |
| `sw-evaluation` | Computes shortage coverage, cost, CVaR and latency metrics used by the product and ablation runner. | Empty/perfect-case test and measured seven-mode output. |
| `sw-orchestrator` | Runs the transaction boundary across BOM, inventory, SQL, HNSW recall, robust ranking, waves, policy, audit and RVF save. | Denial-with-no-durable-effect test plus live composition check. |
| `sw-api` | Runs Axum health/readiness and versioned plan/audit endpoints, serves the React build, and maps typed errors/correlation IDs. | API, middleware, browser and restart suites. |

Review result: all 25 crates have distinct implemented behavior, a real caller or executable role, and relevant verification. None is an empty marker, repetitive validator, ornamental adapter, or mock-only package. The narrow shared-contract, error, configuration and embedding crates are counted because they define cross-cutting typed/runtime boundaries used throughout the real product, not because they exist as directories.
