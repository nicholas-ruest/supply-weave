#!/usr/bin/env bash
set -euo pipefail
target="${SUPPLY_WEAVE_DATA:-data/runtime}"
case "$target" in data/runtime|/tmp/supply-weave-*) find "$target" -depth -delete 2>/dev/null || true;; *) echo "refusing unsafe reset: $target" >&2; exit 2;; esac
