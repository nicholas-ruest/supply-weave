# ADR-0005: DataFusion

Status: Accepted for the 2026-10-27 candidate.

## Context
Supply Weave must preserve a usable procurement workflow and explicit evidence/failure boundaries. This decision addresses datafusion in the live browser-to-RVF path, not as an ornamental preference.

## Decision
Execute Arrow-backed SQL for regional capacity.

## Consequences
Enterprise boundary is real work. The owning crates, callers and context are listed in the architecture map. The choice creates an observable degradation or rejection path rather than a documentation-only claim.

## Verification
RecordBatch SQL test. The exact candidate also undergoes strict checks, product recording and semantic review. File counts alone do not validate the decision.

## Rejected alternatives
Mock-only adapters, cosmetic wrappers and unexecuted prose were rejected because they cannot prove this behavior in a required journey.
