#!/usr/bin/env bash
set -euo pipefail
for i in 1 2 3; do curl -fsS -o /dev/null -w "run=$i seconds=%{time_total}
" -X POST http://127.0.0.1:18080/api/v1/plans -H 'authorization: Bearer planner-token' -H "idempotency-key: bench-key-$i" -H "x-correlation-id: bench-corr-$i" -H 'content-type: application/json' --data '{"disruption_id":"benchmark","region":"APAC","severity":0.64,"duration_days":21,"demand_units":720,"require_certified":true,"max_concentration":1.0,"spend_limit":85000}'; done
