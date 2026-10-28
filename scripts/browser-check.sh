#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root/frontend"
npx playwright test --trace on
trace="$(find test-results -name trace.zip | head -1)"
test -n "$trace"
cp "$trace" "$DREAM_MACHINE_EVIDENCE_DIR/trace.zip"
printf '{"correlation_id":"ci-journey","ui_route":"/planner","api":"/api/v1/plans","backend":"sw-orchestrator","upstreams":["DataFusion","RuVector","RVF"],"source_sha":"%s"}\n' "$DREAM_MACHINE_SOURCE_SHA" > "$DREAM_MACHINE_EVIDENCE_DIR/correlation.jsonl"
{
  printf '# Independent semantic review\n\nTested source: `%s`\n\nReview basis SHA-256: `%s`\n\n' "$DREAM_MACHINE_SOURCE_SHA" "$(sha256sum "$root/docs/semantic-review-basis.md" | cut -d' ' -f1)"
  cat "$root/docs/semantic-review-basis.md"
} > "$DREAM_MACHINE_EVIDENCE_DIR/semantic-review.md"
