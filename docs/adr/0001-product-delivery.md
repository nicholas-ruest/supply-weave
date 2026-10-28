# ADR-0001: Product delivery

Status: Accepted for the 2026-10-27 candidate.

## Context
Supply Weave must preserve a usable procurement workflow and explicit evidence/failure boundaries. This decision addresses product delivery in the live browser-to-RVF path, not as an ornamental preference.

## Decision
Build a navigable procurement workstation, not CLI/generic form.

## Consequences
Four domain views and real browser journeys. The owning crates, callers and context are listed in the architecture map. The choice creates an observable degradation or rejection path rather than a documentation-only claim.

## Verification
Disconnected UI fails. The exact candidate also undergoes strict checks, product recording and semantic review. File counts alone do not validate the decision.

## Rejected alternatives
Mock-only adapters, cosmetic wrappers and unexecuted prose were rejected because they cannot prove this behavior in a required journey.
