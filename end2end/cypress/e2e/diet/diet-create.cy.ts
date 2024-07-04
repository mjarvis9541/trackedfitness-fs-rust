describe("Navigate to the diet and add a food", () => {
  it("Navigate to the diet page", () => {
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    cy.url().should("include", "/users/testuser-cypress");

    cy.contains("a", "Diet").click();
  });
});
