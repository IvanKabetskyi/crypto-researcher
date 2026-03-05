describe('Authentication', () => {
    beforeEach(() => {
        cy.visit('/login');
    });

    it('shows login form', () => {
        cy.get('[data-cy="email-input"]').should('be.visible');
        cy.get('[data-cy="password-input"]').should('be.visible');
        cy.get('[data-cy="login-button"]').should('be.visible');
        cy.contains('Sign In').should('be.visible');
    });

    it('shows error on invalid credentials', () => {
        cy.get('[data-cy="email-input"]').find('input').type('wrong@email.com');
        cy.get('[data-cy="password-input"]').find('input').type('wrongpassword');
        cy.get('[data-cy="login-button"]').click();
        cy.get('[data-cy="login-error"]').should('contain', 'Invalid email or password');
    });

    it('logs in with valid credentials and redirects to home', () => {
        cy.get('[data-cy="email-input"]').find('input').type('ivankabeckii@gmail.com');
        cy.get('[data-cy="password-input"]').find('input').type('CryptoRes2026!');
        cy.get('[data-cy="login-button"]').click();
        cy.url().should('not.include', '/login');
        cy.contains('Signals').should('be.visible');
    });

    it('redirects to login when not authenticated', () => {
        cy.visit('/');
        cy.url().should('include', '/login');
    });

    it('can logout', () => {
        cy.login('ivankabeckii@gmail.com', 'CryptoRes2026!');
        cy.visit('/');
        cy.get('[data-cy="logout-button"]').click();
        cy.url().should('include', '/login');
    });
});
