#!/usr/bin/env node
// Necessary evidence checks; independent source/product review remains mandatory.
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { closeSync, existsSync, mkdirSync, openSync, readFileSync, readdirSync,
  realpathSync, statSync, writeFileSync } from 'node:fs';
import { isAbsolute, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

export const suites = ['rust', 'frontend', 'api', 'middleware', 'integrations',
  'benchmark', 'ablations', 'containers', 'browser'];
const kinds = ['desktop_screenshot', 'mobile_screenshot', 'browser_report',
  'browser_trace', 'correlation_trace', 'semantic_review'];
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (root, ...args) => execFileSync('git', ['-C', root, ...args],
  { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
const inside = (root, path) => {
  const rel = relative(root, path);
  return rel === '' || (!isAbsolute(rel) && rel !== '..' && !rel.startsWith(`..${sep}`));
};
function source(root) {
  const sha = git(root, 'rev-parse', 'HEAD');
  if (!/^[a-f0-9]{40}$/.test(sha)) throw new Error('Invalid source commit');
  if (git(root, 'status', '--porcelain', '--untracked-files=all')) {
    throw new Error('Source checkout must be clean; keep evidence outside it');
  }
  return sha;
}
function safeFile(root, path) {
  if (typeof path !== 'string' || !path || isAbsolute(path)) throw new Error('Invalid artifact path');
  const target = realpathSync(resolve(root, path));
  if (!inside(realpathSync(root), target) || !statSync(target).isFile()) {
    throw new Error(`Artifact escapes evidence directory: ${path}`);
  }
  return target;
}
function evidenceDirectory(root, evidence) {
  mkdirSync(evidence, { recursive: true });
  if (inside(realpathSync(root), realpathSync(evidence))) {
    throw new Error('Use an external evidence directory to avoid circular commit hashes');
  }
}

export function record(root, evidence, suite, argv) {
  root = realpathSync(root);
  evidence = resolve(evidence);
  if (!suites.includes(suite) || !argv.length) throw new Error('Unknown suite or missing command');
  evidenceDirectory(root, evidence);
  const sourceSha = source(root);
  const id = `${suite}-${randomUUID()}`;
  const log = `${id}.log`;
  const fd = openSync(resolve(evidence, log), 'wx');
  const startedAt = new Date().toISOString();
  let result;
  try {
    result = spawnSync(argv[0], argv.slice(1), {
      cwd: root, shell: false, stdio: ['ignore', fd, fd],
      env: { ...process.env, DREAM_MACHINE_EVIDENCE_DIR: evidence,
        DREAM_MACHINE_SOURCE_SHA: sourceSha },
    });
  } finally { closeSync(fd); }
  let integrityError = null;
  try { if (source(root) !== sourceSha) integrityError = 'Source changed during execution'; }
  catch (error) { integrityError = error.message; }
  const receipt = {
    schema_version: 2, suite, source_sha: sourceSha, argv,
    started_at: startedAt, ended_at: new Date().toISOString(),
    exit_code: result.status, signal: result.signal, error: result.error?.message ?? null,
    integrity_error: integrityError, log, log_sha256: digest(readFileSync(resolve(evidence, log))),
  };
  writeFileSync(resolve(evidence, `${id}.receipt.json`), `${JSON.stringify(receipt, null, 2)}\n`);
  return receipt;
}

function browserJourneys(report) {
  const observed = new Set();
  let count = 0;
  function walk(suite) {
    for (const spec of suite.specs ?? []) {
      for (const test of spec.tests ?? []) {
        count += 1;
        if (test.expectedStatus !== 'passed' || !test.results?.length ||
            test.results.some((result) => result.status !== 'passed')) {
          throw new Error(`Browser test failed, skipped, retried or incomplete: ${spec.title}`);
        }
        for (const journey of ['primary', 'denial-recovery', 'durable-retrieval']) {
          if (spec.title?.includes(`[journey:${journey}]`)) observed.add(journey);
        }
      }
    }
    for (const child of suite.suites ?? []) walk(child);
  }
  walk(report);
  if (report.errors?.length || count < 3 || observed.size !== 3) {
    throw new Error('Executed Playwright report must cover all three named journeys without errors');
  }
}

export function verify(root, evidence, expectedSha) {
  root = realpathSync(root);
  evidence = realpathSync(evidence);
  evidenceDirectory(root, evidence);
  const sha = source(root);
  if (expectedSha && sha !== expectedSha) throw new Error('Requested candidate differs from checkout HEAD');
  const missing = [];
  for (const path of ['Cargo.toml', 'Cargo.lock', 'frontend/package.json',
    'frontend/package-lock.json', 'docs/prototype-acceptance.md', 'scripts/verify-prototype.sh']) {
    if (!existsSync(resolve(root, path))) missing.push(path);
  }
  if (missing.length) throw new Error(`NONCOMPLIANT_BUILD: missing product layers: ${missing.join(', ')}`);
  const manifest = JSON.parse(readFileSync(resolve(evidence, 'manifest.json'), 'utf8'));
  if (manifest.schema_version !== 2 || manifest.source_sha !== sha) {
    throw new Error('Unbound/stale evidence: schema_version 2 and exact tested source_sha required');
  }
  const receipts = readdirSync(evidence).filter((name) => name.endsWith('.receipt.json'))
    .map((name) => JSON.parse(readFileSync(safeFile(evidence, name), 'utf8')));
  for (const receipt of receipts) {
    if (receipt.schema_version !== 2 || receipt.source_sha !== sha || receipt.exit_code !== 0 ||
        receipt.signal || receipt.error || receipt.integrity_error ||
        !Array.isArray(receipt.argv) || !receipt.argv.length ||
        !Number.isFinite(Date.parse(receipt.started_at)) ||
        !Number.isFinite(Date.parse(receipt.ended_at)) ||
        Date.parse(receipt.ended_at) < Date.parse(receipt.started_at)) {
      throw new Error('Failed, stale or invalid execution receipt');
    }
    if (digest(readFileSync(safeFile(evidence, receipt.log))) !== receipt.log_sha256) {
      throw new Error('Execution log hash mismatch');
    }
  }
  for (const suite of suites) {
    if (!receipts.some((receipt) => receipt.suite === suite)) throw new Error(`Missing executed suite: ${suite}`);
  }
  const artifacts = manifest.artifacts;
  if (!Array.isArray(artifacts)) throw new Error('Manifest must list hashed evidence artifacts');
  for (const kind of kinds) {
    const artifact = artifacts.find((item) => item.kind === kind);
    if (!artifact) throw new Error(`Missing evidence artifact: ${kind}`);
    const bytes = readFileSync(safeFile(evidence, artifact.path));
    if (!/^[a-f0-9]{64}$/.test(artifact.sha256 ?? '') || digest(bytes) !== artifact.sha256 || !bytes.length) {
      throw new Error(`Missing, empty or hash-mismatched artifact: ${kind}`);
    }
    if (kind.endsWith('_screenshot')) {
      if (bytes.length < 24 || !bytes.subarray(0, 8).equals(Buffer.from('89504e470d0a1a0a', 'hex'))) {
        throw new Error('Screenshot must be actual PNG bytes');
      }
      const width = bytes.readUInt32BE(16);
      const height = bytes.readUInt32BE(20);
      if (height < 480 || (kind === 'desktop_screenshot' ? width < 1024 : width < 320 || width > 600)) {
        throw new Error('Screenshot dimensions do not match desktop/mobile evidence');
      }
    }
    if (kind === 'browser_trace' && !bytes.subarray(0, 2).equals(Buffer.from('PK'))) {
      throw new Error('Browser trace must be a Playwright ZIP');
    }
    if (kind === 'browser_report') browserJourneys(JSON.parse(bytes.toString('utf8')));
    if (kind === 'semantic_review' && !bytes.toString('utf8').includes(sha)) {
      throw new Error('Semantic review is not bound to the tested source commit');
    }
  }
  return { status: 'EVIDENCE_CHECKS_PASS', source_sha: sha, suites: suites.length,
    limitation: 'Necessary checks only. Independently review crate substance, real call paths, upstream execution, UI quality and CI provenance before BUILT.' };
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    const [mode, root, evidence, ...rest] = process.argv.slice(2);
    if (!root || !evidence || !['record', 'verify'].includes(mode)) {
      throw new Error('Usage: gate record ROOT EVIDENCE SUITE -- COMMAND [ARGS] | gate verify ROOT EVIDENCE [SHA]');
    }
    if (mode === 'record') {
      if (rest[1] !== '--') throw new Error('Separate suite and command with --');
      const receipt = record(root, evidence, rest[0], rest.slice(2));
      console.log(JSON.stringify(receipt));
      process.exitCode = receipt.integrity_error || receipt.error || receipt.signal ? 1 : receipt.exit_code ?? 1;
    } else console.log(JSON.stringify(verify(root, evidence, rest[0]), null, 2));
  } catch (error) {
    console.error(JSON.stringify({ status: 'NONCOMPLIANT_BUILD', error: error.message }));
    process.exitCode = 1;
  }
}
