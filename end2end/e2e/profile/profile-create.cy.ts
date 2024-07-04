describe("Create a user profile", () => {
  it("should be able to create a user profile", () => {
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    cy.url().should("include", "/users/testuser-cypress");

    cy.contains("a", "Set up profile").click();

    cy.get("form").should("be.visible");

    cy.get('select[name="sex"]').select("Male");

    cy.get('select[name="activity_level"]').select(
      "Very Active - Heavy exercise/sports 6-7 days a week"
    );

    cy.get('select[name="fitness_goal"]').select("Lose Weight");

    cy.get('input[name="height"]').type("180");

    cy.get('input[name="weight"]').type("75.5");

    cy.get('input[name="date_of_birth"]').type("1990-01-01");

    cy.contains("button", "Create Profile").click();

    cy.url().should("include", "/users/testuser-cypress");
  });
});
