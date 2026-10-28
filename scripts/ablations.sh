#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
cargo build --locked -p sw-api
node scripts/ablation-runner.mjs
