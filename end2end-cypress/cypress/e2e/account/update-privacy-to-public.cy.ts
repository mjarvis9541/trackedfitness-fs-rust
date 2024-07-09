describe("Login and update account settings", () => {
  it("Logs in, navigates to account settings and updates privacy level to private", () => {
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    cy.url().should("include", "/users/testuser-cypress");

    cy.get("#btn-top-nav-side-menu").click();

    cy.get("a").contains("Settings").click();

    cy.get('select[name="privacy_level"]').select("1");

    cy.contains("button", "Update Settings").click();
  });
});
