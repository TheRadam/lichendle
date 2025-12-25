import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page, baseURL }) => {
  await page.goto(baseURL!);
});

test('has title', async ({ page }) => {
  await expect(page).toHaveTitle("Not Lichendle");
});
