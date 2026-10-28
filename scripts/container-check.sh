#!/usr/bin/env bash
set -euo pipefail
docker build -t supply-weave:test .
cid="$(docker run -d -p 18081:8080 -e PORT=8080 supply-weave:test)"
trap 'docker logs "$cid"; docker rm -f "$cid"' EXIT
for i in $(seq 1 60); do curl -fsS http://127.0.0.1:18081/health && exit 0; sleep 1; done
exit 1
