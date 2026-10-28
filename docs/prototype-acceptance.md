# Prototype acceptance

|Journey|UI|API/middleware|Rust domains|Upstream/store|Visible result|
|---|---|---|---|---|---|
|Primary|Planner / Generate|POST v1; bearer, idempotency, correlation|BOM, inventory, robust rank, wave, policy|DataFusion SQL, RuVector recall, RVF append|metrics, waves, tiers, audit|
|Denial/recovery|concentration 10%, then 100%|403 Problem then 201|policy before repository|RVF count unchanged then full composition|error then plan|
|Durable retrieval|Audit / refresh|GET list/get|repository reload and index rebuild|RVF reopen, RuVector reconstruction|saved card and trace|

Playwright does not intercept '/api'. CI captures real desktop/mobile PNGs, JSON report, trace ZIP, correlation JSONL and semantic review outside the checkout and binds them to the exact source SHA.
