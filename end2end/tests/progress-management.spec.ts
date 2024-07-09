import { test, expect } from "@playwright/test";
import { login } from "../utils/login";

test("can create progress if not exists", async ({ page }) => {
  await login(page);

  const link = page.getByRole("link", { name: "Log Progress" });
  const linkVisibile = await link.isVisible();

  if (linkVisibile === true) {
    await link.click();

    await expect(page).toHaveTitle(/Log Progress/);

    await page.getByLabel("Weight (kg)").fill("100");
    await page.getByLabel("Energy Burnt (kcal)").fill("2500");
    await page.getByLabel("notes").fill("Logging some progress for the day");

    await page.getByRole("button", { name: "Log Progress" }).click();
    await expect(page).toHaveTitle(/User Detail/);

    await page.locator("#progress-detail").click();
    await expect(page).toHaveTitle(/Progress/);

    const weightTd = page.locator(
      'tr:has(td:has-text("Weight")) td.text-right'
    );
    const energyTd = page.locator(
      'tr:has(td:has-text("Energy Burnt")) td.text-right'
    );
    const noteTd = page.locator('tr:has(td:has-text("Notes")) td.text-right');

    await expect(weightTd).toContainText("100");
    await expect(energyTd).toContainText("2500");
    await expect(noteTd).toContainText("Logging some progress for the day");
  }
});

test("can view progress if exists", async ({ page }) => {
  await login(page);

  const detailLink = page.locator("#progress-detail");
  const detailLinkVisibile = await detailLink.isVisible();

  if (detailLinkVisibile === true) {
    await detailLink.click();
    await expect(page).toHaveTitle(/Progress/);
  }
});

test("can update progress if exists", async ({ page }) => {
  await login(page);

  const detailLink = page.locator("#progress-detail");
  const detailLinkVisibile = await detailLink.isVisible();

  if (detailLinkVisibile === true) {
    await detailLink.click();
    await expect(page).toHaveTitle(/Progress/);

    await page.getByRole("link", { name: "Edit" }).click();
    await expect(page).toHaveTitle(/Edit Progress/);

    await page.getByLabel("Weight (kg)").fill("105");
    await page.getByLabel("Energy Burnt (kcal)").fill("3000");
    await page.getByLabel("notes").fill("Updating some progress for the day");

    await page.getByRole("button", { name: "Update Progress" }).click();
    await expect(page).toHaveTitle(/User Detail/);

    await page.locator("#progress-detail").click();
    await expect(page).toHaveTitle(/Progress/);

    const weightTd = page.locator(
      'tr:has(td:has-text("Weight")) td.text-right'
    );
    const energyTd = page.locator(
      'tr:has(td:has-text("Energy Burnt")) td.text-right'
    );
    const noteTd = page.locator('tr:has(td:has-text("Notes")) td.text-right');

    await expect(weightTd).toContainText("105");
    await expect(energyTd).toContainText("3000");
    await expect(noteTd).toContainText("Updating some progress for the day");
  }
});

test("can delete progress if exists", async ({ page }) => {
  await login(page);

  const detailLink = page.locator("#progress-detail");
  const detailLinkVisibile = await detailLink.isVisible();

  if (detailLinkVisibile === true) {
    await detailLink.click();
    await expect(page).toHaveTitle(/Progress/);

    await page.getByRole("link", { name: "Delete" }).click();
    await expect(page).toHaveTitle(/Delete Progress/);

    await page.getByRole("button", { name: "Delete Progress" }).click();

    await expect(
      page.getByRole("heading", {
        name: "Progress",
        exact: true,
      })
    ).toBeVisible();
  }
});
