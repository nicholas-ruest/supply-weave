#!/usr/bin/env bash
set -euo pipefail

cargo test --locked -p sw-datafusion -p sw-ruvector -p sw-rvf -p sw-orchestrator -- --nocapture

response="$(curl -fsS -X POST http://127.0.0.1:18080/api/v1/plans \
  -H 'authorization: Bearer planner-token' \
  -H 'content-type: application/json' \
  -H 'idempotency-key: integration-composition' \
  -d '{"scenario_id":"integration-composition","site":"Toronto","disrupted_supplier":"northstar-copper","demand_multiplier":1.25,"max_spend":50000,"risk_aversion":0.8}')"

node -e '
const body=JSON.parse(process.argv[1]);
if(body.status!=="authorized") throw new Error(`unexpected status ${body.status}`);
const audit=body.audit.join("\n");
for(const marker of ["DataFusion capacity=", "RuVector", "DRO", "RVF append"]){
  if(!audit.includes(marker)) throw new Error(`missing executed integration marker: ${marker}`);
}
if(!body.metrics || body.metrics.shortage_coverage <= 0) throw new Error("missing measured outcome");
' "$response"
