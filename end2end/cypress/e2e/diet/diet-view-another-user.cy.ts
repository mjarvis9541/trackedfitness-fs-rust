describe("View another user's diet page", () => {
  it("Navigate to the diet page", () => {
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    cy.url().should("include", "/users/testuser-cypress");

    cy.visit("/users/michael/diet");
  });
});
