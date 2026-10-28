#!/usr/bin/env bash
set -euo pipefail
curl -fsS http://127.0.0.1:18080/health | grep -q 'supply-weave'
curl -fsS http://127.0.0.1:18080/ready | grep -q 'ready|ok'
curl -sS -o /tmp/sw-invalid.json -w '%{http_code}' -X POST http://127.0.0.1:18080/api/v1/plans -H 'authorization: Bearer planner-token' -H 'idempotency-key: invalid-123' -H 'content-type: application/json' --data '{"disruption_id":"x","region":"","severity":2,"duration_days":0,"demand_units":0,"require_certified":true,"max_concentration":1,"spend_limit":1}' | grep -q 422
