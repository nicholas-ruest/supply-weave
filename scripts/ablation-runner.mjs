import { spawn } from 'node:child_process';
import { mkdtemp, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

const root = resolve(new URL('..', import.meta.url).pathname);
const binary = join(root, 'target/debug/sw-api');
const modes = ['combined', 'ruvnet-only', 'enterprise-only', 'no-ruvector', 'no-rvf', 'no-datafusion', 'no-dro'];

async function waitFor(url) {
  for (let attempt = 0; attempt < 80; attempt += 1) {
    try {
      if ((await fetch(`${url}/ready`)).ok) return;
    } catch {}
    await new Promise((resolvePromise) => setTimeout(resolvePromise, 125));
  }
  throw new Error(`backend did not become ready: ${url}`);
}

async function run(mode, index) {
  const directory = await mkdtemp(join(tmpdir(), `supply-weave-${mode}-`));
  const port = 18100 + index;
  const url = `http://127.0.0.1:${port}`;
  const env = {
    ...process.env,
    PORT: String(port),
    SUPPLY_WEAVE_DATA: directory,
    SUPPLY_WEAVE_ABLATION: mode,
  };
  let child = spawn(binary, [], { cwd: root, env, stdio: 'ignore' });
  try {
    await waitFor(url);
    const request = {
      disruption_id: 'ablation-port-closure',
      region: 'APAC',
      severity: 0.64,
      duration_days: 21,
      demand_units: 720,
      require_certified: true,
      max_concentration: 1,
      spend_limit: 85000,
    };
    const create = async (suffix) => {
      const response = await fetch(`${url}/api/v1/plans`, {
        method: 'POST',
        headers: {
          authorization: 'Bearer planner-token',
          'content-type': 'application/json',
          'idempotency-key': `${mode}-${suffix}`,
          'x-correlation-id': `${mode}-${suffix}-corr`,
        },
        body: JSON.stringify(request),
      });
      if (!response.ok) throw new Error(`${mode} failed: ${response.status} ${await response.text()}`);
      return response.json();
    };
    await create('warm');
    const plan = await create('evaluate');
    child.kill('SIGTERM');
    await new Promise((resolvePromise) => child.once('exit', resolvePromise));

    child = spawn(binary, [], { cwd: root, env, stdio: 'ignore' });
    await waitFor(url);
    const saved = await (await fetch(`${url}/api/v1/plans`)).json();
    const durable = saved.length > 0 ? 1 : 0;
    const datafusion = plan.audit.some((event) => event.includes('DataFusion capacity=')) ? 1 : 0;
    const dro = plan.audit.some((event) => event.includes('DRO waves=')) ? 1 : 0;
    const analogues = plan.analogues.length;
    const averageTail = plan.candidates.reduce((total, candidate) => total + candidate.tail_loss, 0)
      / Math.max(1, plan.candidates.length);
    const robustness = dro ? 10 / (1 + averageTail / 100) : 0;
    const utility = plan.metrics.coverage * 70 + robustness + datafusion * 5 + durable * 5 + Math.min(analogues, 2) * 3;
    return {
      mode,
      coverage: plan.metrics.coverage,
      average_tail: averageTail,
      datafusion,
      analogues,
      dro,
      durable,
      utility,
    };
  } finally {
    if (!child.killed) child.kill('SIGTERM');
    await rm(directory, { recursive: true, force: true });
  }
}

const rows = [];
for (const [index, mode] of modes.entries()) rows.push(await run(mode, index));
console.log('mode,coverage,average_tail,datafusion,analogues,dro,durable,utility');
for (const row of rows) {
  console.log([
    row.mode,
    row.coverage.toFixed(3),
    row.average_tail.toFixed(3),
    row.datafusion,
    row.analogues,
    row.dro,
    row.durable,
    row.utility.toFixed(3),
  ].join(','));
}
const combined = rows[0];
if (!rows.slice(1).every((row) => combined.utility > row.utility)) {
  throw new Error('combined mode must outperform every ingredient ablation');
}
if (combined.datafusion !== 1 || combined.analogues < 1 || combined.dro !== 1 || combined.durable !== 1) {
  throw new Error('combined mode did not execute every required ingredient');
}
