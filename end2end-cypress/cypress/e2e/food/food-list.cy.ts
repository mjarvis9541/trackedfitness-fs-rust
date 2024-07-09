describe("View the food list page", () => {
  it("Navigates to home screen and logs in, and views the food list", () => {
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    cy.url().should("include", "/users/testuser-cypress");

    cy.contains("a", "Food").click();

    cy.contains("h1", "Food").should("be.visible");
  });
});
