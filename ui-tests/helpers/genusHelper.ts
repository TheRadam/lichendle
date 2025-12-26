import {expect} from "@playwright/test";


export function getGenusInput(page: any) {
    return page.getByPlaceholder('Genus');
}

export async function fillCorrectGenus(page: any, input: string) {
    let genusInput = getGenusInput(page);
    await genusInput.fill(input);
}

export async function assertCorrectGenus(page: any) {
    let genusInput = getGenusInput(page);
    await expect(genusInput).toContainClass('border-emerald-600');
}

export async function assertIncorrectGenus(page: any) {
    let genusInput = getGenusInput(page);
    await expect(genusInput).toContainClass('border-red-600');
}
