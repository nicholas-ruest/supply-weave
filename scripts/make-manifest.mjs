import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const evidence = process.argv[2];
const sourceSha = process.argv[3];
const files = [
  ['desktop_screenshot', 'desktop.png'],
  ['mobile_screenshot', 'mobile.png'],
  ['browser_report', 'browser-report.json'],
  ['browser_trace', 'trace.zip'],
  ['correlation_trace', 'correlation.jsonl'],
  ['semantic_review', 'semantic-review.md'],
];
const artifacts = files.map(([kind, path]) => ({
  kind,
  path,
  sha256: createHash('sha256').update(readFileSync(join(evidence, path))).digest('hex'),
}));
const manifest = {
  schema_version: 2,
  source_sha: sourceSha,
  generated_at: new Date().toISOString(),
  launch_mode: 'GitHub Actions Ubuntu hosted runner, real Rust backend serving built React frontend',
  test_namespace: 'supply-weave-ci',
  toolchains: { rust: '1.99.1', node: process.version, playwright: '1.56.1' },
  artifacts,
};
writeFileSync(join(evidence, 'manifest.json'), `${JSON.stringify(manifest, null, 2)}\n`);
