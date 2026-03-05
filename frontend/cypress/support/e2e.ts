Cypress.Commands.add('login', (email: string, password: string) => {
    cy.request({
        method: 'POST',
        url: `${Cypress.env('API_URL')}/auth/login`,
        body: { email, password },
    }).then((response) => {
        localStorage.setItem('token', response.body.token);
        localStorage.setItem('email', response.body.email);
    });
});

declare global {
    namespace Cypress {
        interface Chainable {
            login(email: string, password: string): Chainable<void>;
        }
    }
}

export {};
