describe("Navigate to diet target detail and delete", () => {
  it("should navigate to the login screen, log in, and delete the diet target", () => {
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    cy.url().should("include", "/users/testuser-cypress");

    cy.get("#diet-delete").click();

    cy.contains("p.mb-4", "Are you sure you wish to delete this diet target?");

    cy.contains("button", "Delete Diet Target").click();

    cy.url().should("include", "/users/testuser-cypress");
  });
});
