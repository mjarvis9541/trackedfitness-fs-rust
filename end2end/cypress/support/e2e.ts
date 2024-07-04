// ***********************************************************
// This example support/e2e.js is processed and
// loaded automatically before your test files.
//
// This is a great place to put global configuration and
// behavior that modifies Cypress.
//
// You can change the location of this file or turn off
// automatically serving support files with the
// 'supportFile' configuration option.
//
// You can read more here:
// https://on.cypress.io/configuration
// ***********************************************************

// Import commands.js using ES2015 syntax:
import "./commands";

// Alternatively you can use CommonJS syntax:
// require('./commands')

Cypress.on("uncaught:exception", (err, runnable) => {
  // Log detailed error information to the console
  // console.error("Uncaught Exception:", err);
  // console.error("Stack Trace:", err.stack);

  return false;
});

// // Capture unhandled promise rejections
// window.addEventListener("unhandledrejection", (event) => {
//   // Log the reason of the rejection
//   console.error("Unhandled Promise Rejection:", event.reason);
//   event.preventDefault();
// });
