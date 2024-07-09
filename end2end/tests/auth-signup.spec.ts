import { test, expect } from "@playwright/test";

test("can view signup", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle("Welcome - Trackedfitness");

  await page.getByRole("link", { name: "Get Started" }).click();

  await expect(page).toHaveTitle(/Sign up/);
});
