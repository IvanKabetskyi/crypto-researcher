describe('History Page', () => {
    beforeEach(() => {
        cy.login('ikapustin@icloud.com', 'CryptoRes2026!');
        cy.visit('/history');
    });

    it('shows history page title', () => {
        cy.contains('Prediction History').should('be.visible');
    });

    it('shows filter controls', () => {
        cy.contains('Symbol').should('be.visible');
        cy.contains('Direction').should('be.visible');
        cy.contains('Outcome').should('be.visible');
        cy.contains('Apply').should('be.visible');
    });

    it('shows export CSV button', () => {
        cy.contains('Export CSV').should('be.visible');
    });
});
