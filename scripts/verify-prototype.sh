#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
evidence="${DREAM_MACHINE_EVIDENCE_DIR:-/tmp/supply-weave-evidence-$(git -C "$root" rev-parse HEAD)}"
gate="$root/scripts/dream-machine-product-gate.mjs"
mkdir -p "$evidence"
record(){ node "$gate" record "$root" "$evidence" "$1" -- "${@:2}"; }

cd "$root"
record rust bash -lc 'cargo fmt --check && cargo clippy --locked --all-targets --all-features -- -D warnings && cargo test --workspace --locked && cargo deny check && cargo build --locked -p sw-api'
record frontend bash -lc 'npm --prefix frontend ci && npm --prefix frontend audit --audit-level=high && npm --prefix frontend run typecheck && npm --prefix frontend run lint && npm --prefix frontend run test && npm --prefix frontend run build && npm --prefix frontend run contract'

export SUPPLY_WEAVE_DATA="${SUPPLY_WEAVE_DATA:-/tmp/supply-weave-ci-data}"
export PORT=18080
export SUPPLY_WEAVE_REPO_ROOT="$root"
export SUPPLY_WEAVE_BACKEND_BIN="$root/target/debug/sw-api"
"$SUPPLY_WEAVE_BACKEND_BIN" >"$evidence/server.log" 2>&1 &
server=$!
export SUPPLY_WEAVE_SERVER_PID="$server"
cleanup(){
  kill "$server" 2>/dev/null || true
  if test -f "$evidence/backend-restart.pid"; then kill "$(cat "$evidence/backend-restart.pid")" 2>/dev/null || true; fi
}
trap cleanup EXIT
ready=0
for _ in $(seq 1 90); do
  if curl -fsS http://127.0.0.1:18080/ready >/dev/null; then ready=1; break; fi
  sleep 1
done
test "$ready" -eq 1

record api bash scripts/api-check.sh
record middleware bash scripts/middleware-check.sh
record integrations bash scripts/integration-check.sh
record benchmark bash scripts/benchmark.sh
record ablations bash scripts/ablations.sh
record browser bash scripts/browser-check.sh
record containers bash scripts/container-check.sh

sha="$(git -C "$root" rev-parse HEAD)"
node "$root/scripts/make-manifest.mjs" "$evidence" "$sha"
node "$gate" verify "$root" "$evidence" "$sha"
