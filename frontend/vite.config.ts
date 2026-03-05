import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
    plugins: [react()],
    base: process.env.GITHUB_PAGES ? '/crypto-researcher/' : '/',
    define: {
        __API_URL__: JSON.stringify(process.env.VITE_API_URL || 'http://localhost:8080/api'),
    },
    server: {
        proxy: {
            '/api': {
                target: 'http://localhost:8080',
                changeOrigin: true,
            },
        },
    },
    resolve: {
        alias: {
            api: path.resolve(__dirname, './src/api'),
            pages: path.resolve(__dirname, './src/pages'),
            store: path.resolve(__dirname, './src/store'),
            types: path.resolve(__dirname, './src/types'),
        },
    },
});
