describe("Navigate to diet target detail and delete", () => {
  it("should navigate to the login screen, log in, and delete the diet target", () => {
    // Log in using custom command
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    // Verify that the URL includes the user's profile page
    cy.url().should("include", "/users/testuser-cypress");

    // Click the delete button for the diet target
    cy.get("#diet-detail").click();

    // Verify redirect to user's profile page
    cy.url().should("include", "/users/testuser-cypress");
  });
});
