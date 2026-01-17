import { test, expect } from '@playwright/test';
import { assertCorrectGenus, assertIncorrectGenus, fillCorrectGenus } from "../helpers/genusHelper";
import { assertCorrectSpecies, assertIncorrectSpecies, fillSpeciesInput } from "../helpers/speciesHelper";
import {assertIncorrectResult, assertCorrectResult, clickUntilFail} from "../helpers/assertions"
import { CONSTANTS } from '../constants';

test.beforeEach(async ({ page, baseURL }) => {
  await page.goto(baseURL!);
});

test('has title', async ({ page }) => {
  await expect(page).toHaveTitle("Lichendle");
});

test('empty input should fail', async ({ page }) => {
  await clickUntilFail(page)
  await assertIncorrectGenus(page);
  await assertIncorrectSpecies(page);
  await assertIncorrectResult(page);
});

test('correct input should succeed', async ({ page }) => {
  await fillCorrectGenus(page, CONSTANTS.CORRECT_GENUS);
  await fillSpeciesInput(page, CONSTANTS.CORRECT_SPECIES);

  await page.getByText("0/5").click();

  await assertCorrectResult(page);
  await assertCorrectGenus(page);
  await assertCorrectSpecies(page);
});

test('correct genus input should fail', async ({ page }) => {
  await fillCorrectGenus(page, CONSTANTS.CORRECT_GENUS);
  await fillSpeciesInput(page, CONSTANTS.INCORRECT_SPECIES);

  await clickUntilFail(page)
  await assertIncorrectResult(page);
  await assertIncorrectSpecies(page);
  await assertCorrectGenus(page);
});

test('correct species input should fail', async ({ page }) => {
  await fillCorrectGenus(page, CONSTANTS.INCORRECT_GENUS);
  await fillSpeciesInput(page, CONSTANTS.CORRECT_SPECIES);

  await clickUntilFail(page)
  await assertIncorrectResult(page);
  await assertIncorrectGenus(page);
  await assertCorrectSpecies(page);
});