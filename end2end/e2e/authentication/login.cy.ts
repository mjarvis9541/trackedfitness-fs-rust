describe("Navigation and Login Test", () => {
  it("Navigates to home screen and logs in", () => {
    cy.visit("/");

    cy.contains(
      "h1",
      "Achieve Your Fitness Goals with Balance and Ease"
    ).should("be.visible");

    cy.contains("a", "Log in").click();

    cy.get('input[name="email"]').type("testuser-cypress@example.com");
    cy.get('input[name="password"]').type("testuser-cypress");

    cy.contains("button", "Log in").click();

    cy.url().should("include", "/users/testuser-cypress");
  });
});
