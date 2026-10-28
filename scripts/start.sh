#!/usr/bin/env bash
set -euo pipefail
npm --prefix frontend run build
exec cargo run --locked -p sw-api
