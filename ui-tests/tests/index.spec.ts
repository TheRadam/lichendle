import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  console.info('process env', process.env.PLAYWRIGHT_TEST_BASE_URL);
  await page.goto("file://html/html/index.html");
});

test('has title', async ({ page }) => {
  await expect(page).toHaveTitle("Lichendle");
});
