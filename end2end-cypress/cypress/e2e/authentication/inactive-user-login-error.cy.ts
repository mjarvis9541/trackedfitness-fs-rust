describe("Navigation and Login Test", () => {
  it("Navigates to home screen and logs in as an inactive user", () => {
    cy.visit("/");

    // Verify the presence of the homepage title
    cy.contains(
      "h1",
      "Achieve Your Fitness Goals with Balance and Ease"
    ).should("be.visible");

    // Navigate to the login page
    cy.contains("a", "Log in").click();

    // Enter credentials for an inactive user
    cy.get('input[name="email"]').type("testuser-cypress-inactive@example.com");
    cy.get('input[name="password"]').type("testuser-cypress-inactive");

    // Attempt to log in
    cy.contains("button", "Log in").click();

    // Verify that the user is informed of their inactive status
    cy.contains("div", "Invalid credentials").should("be.visible");

    // Verify that the user is not redirected to the home page
    cy.url().should("not.include", "/users/testuser-cypress");
  });
});
