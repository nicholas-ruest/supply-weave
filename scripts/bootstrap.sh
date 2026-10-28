#!/usr/bin/env bash
set -euo pipefail
cargo fetch --locked
npm --prefix frontend ci
npx --yes --prefix frontend playwright install chromium
