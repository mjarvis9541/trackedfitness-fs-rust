describe("Navigate to diet target detail page", () => {
  it("should navigate to the login screen, log in, and view the diet target", () => {
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    cy.url().should("include", "/users/testuser-cypress");

    cy.get("#diet-detail").click();

    cy.url().should("include", "/users/testuser-cypress/diet-target");

    cy.contains("h1", "Diet Target").should("be.visible");
  });
});
