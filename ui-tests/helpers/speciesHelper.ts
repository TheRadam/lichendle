import { expect } from "@playwright/test";

export function getSpeciesInput(page: any) {
    return page.getByPlaceholder('Species');
}

export async function fillSpeciesInput(page: any, input: string) {
    let speciesInput = getSpeciesInput(page);
    await speciesInput.fill(input);
}

export async function assertCorrectSpecies(page: any) {
    let speciesInput = getSpeciesInput(page);
    await expect(speciesInput).toContainClass('border-emerald-600');
}

export async function assertIncorrectSpecies(page: any) {
    let speciesInput = getSpeciesInput(page);
    await expect(speciesInput).toContainClass('border-red-600');
}
