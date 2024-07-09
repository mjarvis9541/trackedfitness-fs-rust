import { expect, Page } from "@playwright/test";

export async function login(page: Page) {
  const email = "testuser-cypress@example.com";
  const password = "testuser-cypress";

  await page.goto("/login");

  await page.waitForSelector('input[name="email"]');
  await page.waitForSelector('input[name="password"]');

  await page.fill('input[name="email"]', email);
  await page.fill('input[name="password"]', password);

  await page.getByRole("button", { name: "Log in" }).click();

  await expect(page).toHaveTitle("User Detail - Trackedfitness");
}
