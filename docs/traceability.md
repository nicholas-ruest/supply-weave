# Traceability

|Requirement|Design|Code|Evidence|
|---|---|---|---|
|Primary sourcing|ADR-0001, DDD-04/08|orchestrator/API/React|journey:primary|
|No-effect denial|ADR-0008, DDD-09|policy before repository|journey:denial-recovery|
|Restart retrieval|ADR-0007, DDD-10|RVF/repository/index rebuild|journey:durable-retrieval|
|DataFusion|ADR-0005|sw-datafusion|integration suite|
|RuVector|ADR-0006|sw-ruvector/recall|integration + analogue count|
|Robust mechanism|ADR-0004|sw-robust/substitution|no-dro ablation|
|Typed contract|ADR-0015|OpenAPI/client|drift check|
|Product evidence|ADR-0017/23|copied gate/verify|external exact-SHA artifact|
|25 crates|ADR-0024|crate map|semantic review|
