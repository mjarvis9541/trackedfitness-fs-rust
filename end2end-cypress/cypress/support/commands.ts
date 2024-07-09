// ***********************************************
// This example commands.js shows you how to
// create various custom commands and overwrite
// existing commands.
//
// For more comprehensive examples of custom
// commands please read more here:
// https://on.cypress.io/custom-commands
// ***********************************************
//
//
// -- This is a parent command --
// Cypress.Commands.add('login', (email, password) => { ... })
//
//
// -- This is a child command --
// Cypress.Commands.add('drag', { prevSubject: 'element'}, (subject, options) => { ... })
//
//
// -- This is a dual command --
// Cypress.Commands.add('dismiss', { prevSubject: 'optional'}, (subject, options) => { ... })
//
//
// -- This will overwrite an existing command --
// Cypress.Commands.overwrite('visit', (originalFn, url, options) => { ... })

declare namespace Cypress {
  interface Chainable {
    login(email: string, password: string): Chainable<Element>;
  }
}

Cypress.Commands.add("login", (email: string, password: string) => {
  cy.visit("/");
  cy.contains("a", "Log in").click();
  cy.get('input[name="email"]').type(email);
  cy.get('input[name="password"]').type(password);
  cy.contains("button", "Log in").click();
});
