describe('Signals Page', () => {
    beforeEach(() => {
        cy.login('ikapustin@icloud.com', 'CryptoRes2026!');
        cy.visit('/');
    });

    it('shows the signal form', () => {
        cy.contains('Analyze').should('be.visible');
        cy.contains('Signals').should('be.visible');
    });

    it('shows trading pair chips', () => {
        cy.contains('BTCUSDT').should('be.visible');
        cy.contains('ETHUSDT').should('be.visible');
    });

    it('shows timeframe selector', () => {
        cy.contains('TF').should('be.visible');
    });

    it('shows empty state message', () => {
        cy.contains('No predictions yet').should('be.visible');
    });
});
