import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page, baseURL }) => {
  const fs = require('fs');

  fs.readdirSync(new URL("file://home/runner/work/lichendle/lichendle/html/").pathname).forEach(file => {
    // will also include directory names
    console.log(file);
  });

  await page.goto(baseURL!);
});

test('has title', async ({ page }) => {
  await expect(page).toHaveTitle("Lichendle");
});
