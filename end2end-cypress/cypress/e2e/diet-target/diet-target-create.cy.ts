describe("Create new diet target", () => {
  it("should navigate to the home screen, log in, and create a new diet target", () => {
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    cy.url().should("include", "/users/testuser-cypress");

    cy.contains("a", "New Diet Target").click();

    cy.get('input[name="weight"]').type("100");

    cy.get('input[name="protein_per_kg"]').type("2.5");

    cy.get('input[name="carbohydrate_per_kg"]').type("5");

    cy.get('input[name="fat_per_kg"]').type("1");

    cy.contains("button", "Create Diet Target").click();
  });
});
