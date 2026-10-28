# Specification

Supply Weave helps procurement resilience, manufacturing, supplier-quality and operations teams translate a regional supplier disruption through a multi-tier bill of materials into an auditable sourcing recovery plan. Spreadsheets hide assumptions, flatten lead-time uncertainty and lose the connection between an order and its authorization evidence.

## Capability and contexts
The product combines Supply Network, Demand and Inventory, Scenario Intelligence, Robust Allocation, Governance, and Evidence and Recovery. Apache DataFusion executes regional capacity SQL; RuVector recalls analogous plans; CVaR-aware robust ranking changes supplier choices; policy denies uncertified or concentrated plans before persistence; RVF durably stores authorized plan/audit records.

## Acceptance journeys
1. **Primary:** '/planner' submits a realistic APAC port closure through typed middleware. The live Rust path executes all ingredients and renders shortages, coverage, cost, tail risk, supplier waves and audit events.
2. **Denial/recovery:** a 10% concentration limit is denied with no new RVF record. A corrected bound recomputes and succeeds.
3. **Durable retrieval:** '/audit' reopens the saved plan after refresh and backend restart from RVF.

## Success
All three real browser journeys pass without first-party route interception; denied append delta is zero; seed coverage is at least 95%; plan P95 is under 1.5 seconds; combined beats enterprise-only coverage and no-DRO tail loss; no-RVF loses retrieval.

## Non-goals
No live purchasing, supplier-master mutation, production deployment or spend is authorized. Seed records are synthetic. The bounded transparent robust heuristic is not represented as a globally optimal MILP.
