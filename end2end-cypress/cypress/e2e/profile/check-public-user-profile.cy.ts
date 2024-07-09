describe("Check public user profile visibility", () => {
  it("should navigate to the login screen, log in, and view a public user profile", () => {
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    cy.url().should("include", "/users/testuser-cypress");

    cy.visit("/users/testuser-cypress-public");

    cy.contains("button", "Follow").should("exist");
  });
});
