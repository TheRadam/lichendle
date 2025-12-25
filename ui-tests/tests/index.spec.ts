import { test, expect } from '@playwright/test';
import fs from "fs";

test.beforeEach(async ({ page, baseURL }) => {
  const fs = require('fs');

  fs.readdirSync(new URL("file://home/home/runner/work/lichendle/lichendle").pathname).forEach(file => {
    // will also include directory names
    console.log(file);
  });

  await page.goto(baseURL!);
});

test('has title', async ({ page }) => {
  await expect(page).toHaveTitle("Lichendle");
});
