describe("Navigate to diet target update page and update", () => {
  it("should navigate to the login screen, log in, and update the diet target", () => {
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    cy.url().should("include", "/users/testuser-cypress");

    cy.get("#diet-detail").click();

    cy.contains("a", "Edit").click();

    cy.get('input[name="weight"]').clear().type("105");

    cy.get('input[name="protein_per_kg"]').clear().type("3.0");

    cy.contains("button", "Update Diet Target").click();

    cy.get("tr").contains("td", "Weight").next().should("contain", "105");
    cy.get("tr")
      .contains("td", "Protein per kg")
      .next()
      .should("contain", "3.0");
  });
});
