describe("Can create profile via set up", () => {
  it("should navigate to the login screen, log in, and delete the diet target", () => {
    cy.login("testuser-cypress@example.com", "testuser-cypress");

    cy.url().should("include", "/users/testuser-cypress");

    cy.contains("a", "Set up profile").click();

    cy.get("form").should("be.visible");

    // Select options from dropdowns
    cy.get('select[name="sex"]').select("M");
    cy.get('select[name="activity_level"]').select("VA");
    cy.get('select[name="fitness_goal"]').select("LW"); //

    // Fill out the number inputs
    cy.get('input[name="height"]').type("180"); // Adjust '180' to the desired height
    cy.get('input[name="weight"]').type("75.5"); // Adjust '75.5' to the desired weight

    // Fill out the date input
    cy.get('input[name="date_of_birth"]').type("1990-01-01"); // Adjust '1990-01-01' to the desired date

    // Submit the form
    cy.contains("button", "Create Profile").click();

    // Assert that the form submission was successful
    // This part depends on what happens after form submission, adjust accordingly

    cy.url().should("include", "/users/testuser-cypress");
  });
});
