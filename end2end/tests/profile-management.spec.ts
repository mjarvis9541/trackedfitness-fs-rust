import { test, expect } from "@playwright/test";
import { login } from "../utils/login";

test("can create profile if not exists", async ({ page }) => {
  await login(page);

  const link = page.getByRole("link", { name: "Set up Profile" });
  const linkVisibile = await link.isVisible();

  if (linkVisibile === true) {
    await link.click();

    await expect(page).toHaveTitle(/Set up Profile/);

    await page.getByLabel("Fitness Goal").selectOption({ value: "LW" });
    await page.getByLabel("Activity Level").selectOption({ value: "MA" });
    await page.getByLabel("Sex").selectOption({ value: "M" });
    await page.getByLabel("Height").fill("180");
    await page.getByLabel("Weight", { exact: true }).fill("80");
    await page.getByLabel("Date of Birth").fill("1990-01-01");

    await page.getByRole("button", { name: "Create Profile" }).click();
    await expect(page).toHaveTitle(/User Detail/);
  }
});

test("can view profile if exists", async ({ page }) => {
  await login(page);

  const detailLink = page.locator("#profile-detail");
  const detailLinkVisibile = await detailLink.isVisible();

  if (detailLinkVisibile === true) {
    await detailLink.click();
    await expect(page).toHaveTitle(/Profile/);
  }
});

test("can update profile if exists", async ({ page }) => {
  await login(page);

  const detailLink = page.locator("#profile-detail");
  const detailLinkVisibile = await detailLink.isVisible();

  if (detailLinkVisibile === true) {
    await detailLink.click();
    await expect(page).toHaveTitle(/Profile/);

    await page.getByRole("link", { name: "Edit" }).click();
    await expect(page).toHaveTitle(/Edit Profile/);

    await page.getByLabel("Fitness Goal").selectOption({ value: "GW" });
    await page.getByLabel("Activity Level").selectOption({ value: "VA" });

    await page.getByRole("button", { name: "Update Profile" }).click();
    await expect(page).toHaveTitle(/User Detail/);

    await detailLink.click();
    await expect(page).toHaveTitle(/Profile/);

    const activityTd = page.locator(
      'tr:has(td:has-text("Activity Level")) td.text-right'
    );
    const goalTd = page.locator(
      'tr:has(td:has-text("Fitness Goal")) td.text-right'
    );

    await expect(activityTd).toContainText("Very Active");
    await expect(goalTd).toContainText("Gain Weight");
  }
});

test("can delete profile if exists", async ({ page }) => {
  await login(page);

  const detailLink = page.locator("#profile-detail");
  const detailLinkVisibile = await detailLink.isVisible();

  if (detailLinkVisibile === true) {
    await detailLink.click();
    await expect(page).toHaveTitle(/Profile/);

    await page.getByRole("link", { name: "Delete" }).click();
    await expect(page).toHaveTitle(/Delete Profile/);

    await page.getByRole("button", { name: "Delete Profile" }).click();

    await expect(page).toHaveTitle(/User Detail/);
  }
});
