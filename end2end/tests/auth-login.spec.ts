import { test, expect } from "@playwright/test";

test("can login", async ({ page }) => {
  const email = "testuser-cypress@example.com";
  const password = "testuser-cypress";

  await page.goto("/");

  await expect(page).toHaveTitle("Welcome - Trackedfitness");

  await page.getByRole("link", { name: "Log in" }).nth(1).click();

  await page.waitForSelector('input[name="email"]');
  await page.waitForSelector('input[name="password"]');

  await page.fill('input[name="email"]', email);
  await page.fill('input[name="password"]', password);

  await page.getByRole("button", { name: "Log in" }).click();
});
