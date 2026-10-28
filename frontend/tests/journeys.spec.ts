import { expect, test } from '@playwright/test';
import { spawn } from 'node:child_process';
import { writeFile } from 'node:fs/promises';

const evidence = process.env.DREAM_MACHINE_EVIDENCE_DIR ?? 'test-results';

test('[journey:primary] planner creates and renders a composed recovery plan', async ({ page }) => {
  await page.goto('/');
  await page.getByRole('button', { name: 'Plan disruption' }).click();
  await page.getByRole('button', { name: 'Generate recovery plan' }).click();
  await expect(page.getByText('Staged procurement')).toBeVisible();
  await expect(page.getByText(/production coverage/)).toBeVisible();
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.screenshot({ path: `${evidence}/desktop.png`, fullPage: true });
});

test('[journey:denial-recovery] policy denial has no effect then correction succeeds', async ({ page, request }) => {
  const before = await (await request.get('/api/v1/plans')).json();
  await page.goto('/');
  await page.getByRole('button', { name: 'Plan disruption' }).click();
  await page.getByLabel(/Max supplier concentration/).fill('0.1');
  await page.getByRole('button', { name: 'Generate recovery plan' }).click();
  await expect(page.getByText(/policy denied|no plan within/i)).toBeVisible();
  const after = await (await request.get('/api/v1/plans')).json();
  expect(after.length).toBe(before.length);
  await page.getByLabel(/Max supplier concentration/).fill('1');
  await page.getByRole('button', { name: 'Generate recovery plan' }).click();
  await expect(page.getByText('Staged procurement')).toBeVisible();
});

test('[journey:durable-retrieval] saved plan survives refresh and actual backend restart', async ({ page }) => {
  const originalPid = Number(process.env.SUPPLY_WEAVE_SERVER_PID);
  const backend = process.env.SUPPLY_WEAVE_BACKEND_BIN;
  if (!Number.isInteger(originalPid) || !backend) throw new Error('restart harness environment is incomplete');

  await page.goto('/');
  await page.getByRole('button', { name: 'Audit & recovery' }).click();
  await expect(page.locator('.saved').first()).toBeVisible();
  await page.reload();
  await page.getByRole('button', { name: 'Audit & recovery' }).click();
  await expect(page.locator('.saved').first()).toBeVisible();

  process.kill(originalPid, 'SIGTERM');
  await new Promise((resolve) => setTimeout(resolve, 750));
  const restarted = spawn(backend, [], {
    cwd: process.env.SUPPLY_WEAVE_REPO_ROOT,
    env: process.env,
    detached: true,
    stdio: 'ignore',
  });
  restarted.unref();
  if (!restarted.pid) throw new Error('backend restart did not return a pid');
  await writeFile(`${evidence}/backend-restart.pid`, `${restarted.pid}\n`);

  await expect.poll(async () => {
    try {
      return (await fetch('http://127.0.0.1:18080/ready')).ok;
    } catch {
      return false;
    }
  }, { timeout: 30_000 }).toBe(true);

  await page.reload();
  await page.getByRole('button', { name: 'Audit & recovery' }).click();
  await expect(page.locator('.saved').first()).toBeVisible();
  await page.setViewportSize({ width: 390, height: 844 });
  await page.screenshot({ path: `${evidence}/mobile.png`, fullPage: true });
});
