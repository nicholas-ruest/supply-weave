# Implementation contract

Codex owns design and implementation on an isolated branch; governed shell commands are execution only. A crate counts only when behavior is called or directly integration-tested and has an explicit failure path. The product requires Axum, React, typed client, middleware, real DataFusion/RuVector/RVF, robust allocation, seed/reset, container, Cloud Run configuration and three browser journeys.

Verification runs from a clean immutable commit. The source-pinned recorder writes nine schema-version-2 suites outside the checkout. CI uploads an artifact named with the tested SHA. EVIDENCE_CHECKS_PASS is necessary; semantic crate review, UI inspection, upstream call-path inspection and exact workflow/artifact readback remain separate gates. Mock first-party routes, JSON shadow persistence, echo suites, placeholder SHAs, stale artifacts and cosmetic crate splitting are forbidden.
