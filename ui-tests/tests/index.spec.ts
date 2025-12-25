import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page, baseURL }) => {
  console.info('process env', process.env.PLAYWRIGHT_TEST_BASE_URL);
  await page.goto(baseURL);
});

test('has title', async ({ page }) => {
  await expect(page).toHaveTitle("Lichendle");
});
