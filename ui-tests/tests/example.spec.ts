import { test, expect } from '@playwright/test';
import path = require("node:path");

test.beforeEach(async ({ page }) => {
  await page.goto(`file:${path.join(__dirname, '../../html/index.html')}`);
});

test('has title', async ({ page }) => {
  await expect(page).toHaveTitle("Lichendle");
});
