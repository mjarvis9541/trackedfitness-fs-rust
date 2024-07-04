/// <reference types="cypress" />

/**
 * Test suite for creating, updating, viewing, and deleting a diet target
 */
describe("Diet Target Management", () => {
  /**
   * Logs in, navigates to the home screen, and creates a new diet target
   */
  it("should create a new diet target", () => {
    // Log in using custom command
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    // Verify that the URL includes the user's profile page
    cy.url().should("include", "/users/testuser-cypress");

    // Navigate to the New Diet Target page
    cy.contains("a", "New Diet Target").click();

    // Fill out the new diet target form
    cy.get('input[name="weight"]').type("100");
    cy.get('input[name="protein_per_kg"]').type("2.5");
    cy.get('input[name="carbohydrate_per_kg"]').type("5");
    cy.get('input[name="fat_per_kg"]').type("1");

    // Submit the form to create the diet target
    cy.contains("button", "Create Diet Target").click();
  });

  /**
   * Logs in and navigates to the diet target detail page
   */
  it("should view the diet target detail", () => {
    // Log in using custom command
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    // Verify that the URL includes the user's profile page
    cy.url().should("include", "/users/testuser-cypress");

    // Navigate to the diet target detail page
    cy.get("#diet-detail").click();

    // Verify redirect to diet target detail page
    cy.url().should("include", "/diet-targets/");
  });

  /**
   * Logs in, navigates to the diet target detail page, and updates the diet target
   */
  it("should update the diet target", () => {
    // Log in using custom command
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    // Verify that the URL includes the user's profile page
    cy.url().should("include", "/users/testuser-cypress");

    // Navigate to the diet target detail page
    cy.get("#diet-detail").click();

    // Click the edit button to update the diet target
    cy.contains("a", "Edit").click();

    // Update the diet target form fields
    cy.get('input[name="weight"]').clear().type("105");
    cy.get('input[name="protein_per_kg"]').clear().type("3.0");

    // Submit the form to update the diet target
    cy.contains("button", "Update Diet Target").click();

    // Verify the updates are reflected on the detail page
    // Verify the updates are reflected in the table rows
    cy.get("tr").contains("td", "Weight").next().should("contain", "105");
    cy.get("tr")
      .contains("td", "Protein per kg")
      .next()
      .should("contain", "3.0");
  });

  /**
   * Logs in, navigates to the diet target detail page, and deletes the diet target
   */
  it("should delete the diet target", () => {
    // Log in using custom command
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    // Verify that the URL includes the user's profile page
    cy.url().should("include", "/users/testuser-cypress");

    // Navigate to the diet target detail page
    cy.get("#diet-detail").click();

    // Click the delete button for the diet target
    cy.contains("a", "Delete").click();

    // Verify the confirmation message appears
    cy.contains("p.mb-4", "Are you sure you wish to delete this diet target?");

    // Confirm deletion by clicking the delete button
    cy.contains("button", "Delete Diet Target").click();

    // Verify redirect to user's profile page
    cy.url().should("include", "/users/testuser-cypress");
  });
});
