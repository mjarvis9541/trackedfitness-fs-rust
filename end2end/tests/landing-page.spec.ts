import { test, expect } from "@playwright/test";

test("can view landing page", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle("Welcome - Trackedfitness");

  await expect(
    page.getByRole("heading", {
      name: "Achieve Your Fitness Goals with Balance and Ease",
      exact: true,
    })
  ).toBeVisible();

  await expect(page.getByRole("link", { name: "Log in" }).nth(1)).toBeVisible();

  await expect(page.getByRole("link", { name: "Get Started" })).toBeVisible();
});

test("can view terms page", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle("Welcome - Trackedfitness");

  await expect(
    page.getByRole("link", { name: "Terms of Service" })
  ).toBeVisible();

  await page.getByRole("link", { name: "Terms of Service" }).click();

  await expect(
    page.getByRole("heading", { name: "Terms of Service", exact: true })
  ).toBeVisible();
});

test("can view privacy page", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle("Welcome - Trackedfitness");

  await expect(
    page.getByRole("link", { name: "Privacy Policy" })
  ).toBeVisible();

  await page.getByRole("link", { name: "Privacy Policy" }).click();

  await expect(
    page.getByRole("heading", { name: "Privacy Policy", exact: true })
  ).toBeVisible();
});
