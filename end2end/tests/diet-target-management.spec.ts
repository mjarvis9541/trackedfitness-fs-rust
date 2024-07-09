import { test, expect } from "@playwright/test";
import { login } from "../utils/login";

test("can create diet target if not exists", async ({ page }) => {
  await login(page);

  const link = page.locator("#diet-target-create");
  const linkVisibile = await link.isVisible();

  if (linkVisibile === true) {
    const weight = "100";
    const protein_per_kg = "2.5";
    const carbohydrate_per_kg = "5";
    const fat_per_kg = "1";

    await link.click();

    await expect(page).toHaveTitle(/New Diet Target/);

    await page.fill('input[name="weight"]', weight);
    await page.fill('input[name="protein_per_kg"]', protein_per_kg);
    await page.fill('input[name="carbohydrate_per_kg"]', carbohydrate_per_kg);
    await page.fill('input[name="fat_per_kg"]', fat_per_kg);

    const button = page.getByRole("button", { name: "Create Diet Target" });
    await expect(button).toBeVisible();
    await button.click();

    await expect(page).toHaveTitle(/Diet Target/);
  }
});

test("can view diet target if exists", async ({ page }) => {
  await login(page);

  const link = page.locator("#diet-target-detail");
  const linkVisibile = await link.isVisible();

  if (linkVisibile === true) {
    await link.click();

    await expect(page).toHaveTitle(/Diet Target/);
  }
});

test("can update diet target if exists", async ({ page }) => {
  await login(page);

  const updated_weight = "105";
  const updated_protein_per_kg = "3.0";

  const link = page.locator("#diet-target-detail");
  const linkVisibile = await link.isVisible();

  if (linkVisibile === true) {
    await link.click();

    const updateLink = page.getByRole("link", { name: "Edit" });
    await expect(updateLink).toBeVisible();
    await updateLink.click();

    await expect(page).toHaveTitle(/Edit Diet Target/);

    await page.fill('input[name="weight"]', updated_weight);
    await page.fill('input[name="protein_per_kg"]', updated_protein_per_kg);

    const updateButton = page.getByRole("button", {
      name: "Update Diet Target",
    });
    await expect(updateButton).toBeVisible();
    await updateButton.click();

    await link.click();

    await expect(page).toHaveTitle(/Diet Target/);

    const weightTd = page.locator(
      'tr:has(td:has-text("Weight")) td.text-right'
    );
    await expect(weightTd).toContainText("105");

    const proteinPerKgTd = page.locator(
      'tr:has(td:has-text("Protein per kg")) td.text-right'
    );
    await expect(proteinPerKgTd).toContainText("3.00");
  }
});

test("can delete diet target if exists", async ({ page }) => {
  await login(page);

  const link = page.locator("#diet-target-detail");
  const linkVisibile = await link.isVisible();

  if (linkVisibile === true) {
    await link.click();

    await expect(page).toHaveTitle(/Diet Target/);

    const deleteLink = page.getByRole("link", { name: "Delete" });
    await expect(deleteLink).toBeVisible();
    await deleteLink.click();

    await expect(page).toHaveTitle(/Delete Diet Target/);

    const deleteButton = page.getByRole("button", {
      name: "Delete Diet Target",
    });
    await expect(deleteButton).toBeVisible();
    await deleteButton.click();

    await expect(page).toHaveTitle(/User Detail/);
  }
});
