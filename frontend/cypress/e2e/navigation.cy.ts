describe('Navigation', () => {
    beforeEach(() => {
        cy.login('ivankabeckii@gmail.com', 'CryptoRes2026!');
    });

    it('navigates to Signals page', () => {
        cy.visit('/');
        cy.get('[data-cy="nav-signals"]').should('be.visible');
        cy.contains('Signals').should('be.visible');
    });

    it('navigates to History page', () => {
        cy.visit('/');
        cy.get('[data-cy="nav-history"]').click();
        cy.url().should('include', '/history');
        cy.contains('Prediction History').should('be.visible');
    });

    it('navigates back to Signals from History', () => {
        cy.visit('/history');
        cy.get('[data-cy="nav-signals"]').click();
        cy.url().should('not.include', '/history');
        cy.contains('Signals').should('be.visible');
    });
});
