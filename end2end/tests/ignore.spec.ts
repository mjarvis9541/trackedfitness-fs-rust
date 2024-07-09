import { test, expect } from "@playwright/test";
import { login } from "../utils/login";

test("progress create, update and delete", async ({ page }) => {
  await login(page);

  await page.getByRole("link", { name: "Log in" }).nth(1).click();
  await page.getByPlaceholder("Enter your email address").click();
  await page
    .getByPlaceholder("Enter your email address")
    .fill("testuser-cypress@example.com");
  await page.getByPlaceholder("Enter your password").fill("testuser-cypress");
  await page.getByPlaceholder("Enter your password").press("Tab");
  await page.getByRole("button", { name: "Log in" }).press("Enter");
  await page.locator("#progress-delete").click();
  await page.getByRole("button", { name: "Delete Progress" }).click();
  await page.getByRole("link", { name: "Log Progress" }).click();
  await page.getByLabel("Weight (kg)").click();
  await page.getByLabel("Weight (kg)").fill("100");
  await page.getByLabel("Weight (kg)").press("Tab");
  await page.getByLabel("Energy Burnt (kcal)").fill("2500");
  await page.getByLabel("Energy Burnt (kcal)").press("Tab");
  await page.getByLabel("notes").press("CapsLock");
  await page.getByLabel("notes").fill("Some progress today");
  await page.getByLabel("notes").press("Tab");
  await page.getByRole("button", { name: "Log Progress" }).press("Enter");
  await page.locator("#progress-detail").click();
  await page.getByRole("link", { name: "Edit" }).click();
  await page.getByPlaceholder("Enter your weight in kg").click();
  await page
    .getByPlaceholder("Enter your weight in kg")
    .press("Alt+Shift+ArrowLeft");
  await page
    .getByPlaceholder("Enter your weight in kg")
    .press("Alt+Shift+ArrowLeft");
  await page.getByPlaceholder("Enter your weight in kg").fill("125");
  await page.getByPlaceholder("Enter your weight in kg").press("Tab");
  await page.getByPlaceholder("Enter energy burnt in kcal").fill("3000");
  await page.getByPlaceholder("Enter energy burnt in kcal").press("Tab");
  await page.getByPlaceholder("Enter any relevant notes").press("Tab");
  await page
    .getByRole("button", { name: "Update Progress" })
    .press("Shift+Tab");
  await page.getByPlaceholder("Enter any relevant notes").press("ArrowLeft");
  await page
    .getByPlaceholder("Enter any relevant notes")
    .press("Alt+ArrowRight");
  await page
    .getByPlaceholder("Enter any relevant notes")
    .fill("Some updated progress today");
  await page.getByPlaceholder("Enter any relevant notes").press("Tab");
  await page.getByRole("button", { name: "Update Progress" }).press("Enter");
  await page.locator("#progress-detail").click();
  await page.getByRole("heading", { name: "Progress" }).click();
  await page.getByRole("link", { name: "Delete" }).click();
  await page.getByRole("button", { name: "Delete Progress" }).click();
  await page.getByText("No progress logged today or").click();
});
