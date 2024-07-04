describe("Check public user profile visibility", () => {
  it("should navigate to the login screen, log in, and delete the diet target", () => {
    // Log in using custom command
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    // Verify that the URL includes the user's profile page
    cy.url().should("include", "/users/testuser-cypress");

    cy.visit("/users/testuser-cypress-public");

    cy.contains("button", "Follow").should("exist");
  });
});

describe("Check private user profile visibility", () => {
  it("should navigate to the login screen, log in, and delete the diet target", () => {
    // Log in using custom command
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    // Verify that the URL includes the user's profile page
    cy.url().should("include", "/users/testuser-cypress");

    cy.visit("/users/testuser-cypress-private");

    cy.contains("button", "Follow").should("exist");
  });
});
