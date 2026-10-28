#!/usr/bin/env bash
set -euo pipefail
payload='{"disruption_id":"auth-check","region":"Toronto","severity":0.5,"duration_days":10,"demand_units":300,"require_certified":true,"max_concentration":0.8,"spend_limit":25000}'
curl -sS -o /tmp/sw-auth.json -w '%{http_code}' -X POST http://127.0.0.1:18080/api/v1/plans -H 'content-type: application/json' --data "$payload" | grep -q 403
curl -sS -o /tmp/sw-viewer.json -w '%{http_code}' -X POST http://127.0.0.1:18080/api/v1/plans -H 'authorization: Bearer viewer-token' -H 'idempotency-key: viewer-123' -H 'content-type: application/json' --data "$payload" | grep -q 403
grep -q 'policy_denied' /tmp/sw-auth.json
grep -q 'policy_denied' /tmp/sw-viewer.json
