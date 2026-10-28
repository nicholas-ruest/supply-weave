#!/usr/bin/env bash
set -euo pipefail
curl -sS -o /tmp/sw-auth.json -w '%{http_code}' -X POST http://127.0.0.1:18080/api/v1/plans -H 'content-type: application/json' --data '{}' | grep -q 403
curl -sS -o /tmp/sw-viewer.json -w '%{http_code}' -X POST http://127.0.0.1:18080/api/v1/plans -H 'authorization: Bearer viewer-token' -H 'idempotency-key: viewer-123' -H 'content-type: application/json' --data '{}' | grep -q 403
