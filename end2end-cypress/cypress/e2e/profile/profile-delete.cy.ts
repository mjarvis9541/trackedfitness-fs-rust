describe("Delete a user profile", () => {
  it("should navigate to the login screen, log in, and delete a user profile", () => {
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    cy.url().should("include", "/users/testuser-cypress");

    cy.get("#profile-delete").click();

    cy.contains("p.mb-4", "Are you sure you wish to delete this profile?");

    cy.contains("button", "Delete Profile").click();

    cy.url().should("include", "/users/testuser-cypress");
  });
});
