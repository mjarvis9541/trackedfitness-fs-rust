describe("Navigation and Login Test", () => {
  it("Navigates to home screen and logs in", () => {
    // Visit the home screen
    cy.visit("/");

    // Check if the home screen is displayed
    cy.contains(
      "h1",
      "Achieve Your Fitness Goals with Balance and Ease"
    ).should("be.visible");

    cy.contains("a", "Log in").click();

    // Fill in the login form using name attributes
    cy.get('input[name="email"]').type("testuser-cypress@example.com");
    cy.get('input[name="password"]').type("testuser-cypress");

    // Click the login button
    cy.contains("button", "Log in").click();

    // Verify the login was successful
    // This will depend on how your application indicates a successful login.
    // For example, you might be redirected to a dashboard or see a welcome message.
    // Adjust the selector and assertion according to your app's behavior.
    cy.url().should("include", "/users/testuser-cypress");
  });
});
