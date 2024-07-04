describe("navigaste to home screen", () => {
  it("Navigates to home screen", () => {
    // Visit the home screen
    cy.visit("/");

    // Check if the home screen is displayed
    cy.contains(
      "h1",
      "Achieve Your Fitness Goals with Balance and Ease"
    ).should("be.visible");
  });
});
