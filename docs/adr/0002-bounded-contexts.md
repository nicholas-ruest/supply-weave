# ADR-0002: Bounded contexts

Status: Accepted for the 2026-10-27 candidate.

## Context
Supply Weave must preserve a usable procurement workflow and explicit evidence/failure boundaries. This decision addresses bounded contexts in the live browser-to-RVF path, not as an ornamental preference.

## Decision
Own network, demand, intelligence, allocation, governance and evidence separately.

## Consequences
Typed APIs and one application transaction. The owning crates, callers and context are listed in the architecture map. The choice creates an observable degradation or rejection path rather than a documentation-only claim.

## Verification
Context call paths inspected. The exact candidate also undergoes strict checks, product recording and semantic review. File counts alone do not validate the decision.

## Rejected alternatives
Mock-only adapters, cosmetic wrappers and unexecuted prose were rejected because they cannot prove this behavior in a required journey.
