describe("Navigate to diet target detail and delete", () => {
  it("should navigate to the login screen, log in, and delete the diet target", () => {
    // Log in using custom command
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    // Verify that the URL includes the user's profile page
    cy.url().should("include", "/users/testuser-cypress");

    // Click the delete button for the diet target
    cy.get("#profile-delete").click();

    // Verify the confirmation message appears
    cy.contains("p.mb-4", "Are you sure you wish to delete this diet target?");

    // Confirm deletion by clicking the delete button
    cy.contains("button", "Delete Diet Target").click();

    // Verify redirect to user's profile page
    cy.url().should("include", "/users/testuser-cypress");
  });
});
