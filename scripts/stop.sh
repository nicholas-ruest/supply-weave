#!/usr/bin/env bash
set -euo pipefail
pkill -f 'target/debug/sw-api' 2>/dev/null || true
