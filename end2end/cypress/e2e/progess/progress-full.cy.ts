/// <reference types="cypress" />

/**
 * Test suite for creating, updating, viewing, and deleting a progress log.
 */
describe("Progress Management", () => {
  // Reusable login function
  const login = () => {
    cy.login("testuser-cypress@example.com", "testuser-cypress");
    cy.url().should("include", "/users/testuser-cypress");
  };

  /**
   * Logs in, navigates to the home screen, and creates a new progress log.
   */
  it("should create a new progress log", () => {
    login();

    cy.contains("a", "Log Progress").click();

    cy.get('input[name="weight_kg"]').type("100");
    cy.get('input[name="energy_burnt"]').type("2500");
    cy.get('input[name="notes"]').type("I've had a good day");

    cy.contains("button", "Log Progress").click();
  });

  /**
   * Logs in and views the progress log detail.
   */
  it("should view the progress log detail", () => {
    login();

    cy.get("#progress-detail").click();

    cy.url().should("include", "/progress/");
  });

  /**
   * Logs in, navigates to the progress log detail page, and updates the progress log.
   */
  it("should update the progress log", () => {
    login();

    cy.get("#progress-detail").click();
    cy.contains("a", "Edit").click();

    cy.get('input[name="weight_kg"]').clear().type("105");
    cy.get('input[name="energy_burnt"]').clear().type("3000");

    cy.contains("button", "Update Progress").click();

    cy.get("tr").contains("td", "Weight").next().should("contain", "105");
    cy.get("tr")
      .contains("td", "Energy Burnt")
      .next()
      .should("contain", "3000");
  });

  /**
   * Logs in, navigates to the progress log detail page, and deletes the progress log.
   */
  it("should delete the progress log", () => {
    login();

    cy.get("#progress-detail").click();
    cy.contains("a", "Delete").click();

    cy.contains(
      "p.mb-4",
      "Are you sure you wish to delete this progress log entry?"
    );
    cy.contains("button", "Delete Progress").click();

    cy.url().should("include", "/users/testuser-cypress");
  });
});
