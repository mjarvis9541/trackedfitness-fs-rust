describe("Check private user profile visibility", () => {
  it("should ensure we can follow a private user", () => {
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    cy.url().should("include", "/users/testuser-cypress");

    cy.visit("/users/testuser-cypress-private");

    cy.contains("button", "Follow").should("exist");
  });
});
