import {CONSTANTS} from "../constants";
import {expect} from "@playwright/test";

export async function assertIncorrectResult(page: any) {
    let result = page.getByText(CONSTANTS.FAILURE_TEXT);
    await expect(result).toContainClass('text-red-600');
}

export async function assertCorrectResult(page: any) {
    let result = page.getByText(CONSTANTS.SUCCESS_TEXT);
    await expect(result).toContainClass('text-emerald-600');
}