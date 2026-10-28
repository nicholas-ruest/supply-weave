# ADR-0014: Auth fixture

Status: Accepted for the 2026-10-27 candidate.

## Context
Supply Weave must preserve a usable procurement workflow and explicit evidence/failure boundaries. This decision addresses auth fixture in the live browser-to-RVF path, not as an ornamental preference.

## Decision
Use deterministic local bearer roles only.

## Consequences
No production credential claims. The owning crates, callers and context are listed in the architecture map. The choice creates an observable degradation or rejection path rather than a documentation-only claim.

## Verification
Viewer denial. The exact candidate also undergoes strict checks, product recording and semantic review. File counts alone do not validate the decision.

## Rejected alternatives
Mock-only adapters, cosmetic wrappers and unexecuted prose were rejected because they cannot prove this behavior in a required journey.
