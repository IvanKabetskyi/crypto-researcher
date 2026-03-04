import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
    plugins: [react()],
    resolve: {
        alias: {
            components: path.resolve(__dirname, './src/components'),
            pages: path.resolve(__dirname, './src/pages'),
            store: path.resolve(__dirname, './src/store'),
            core: path.resolve(__dirname, './src/core'),
            services: path.resolve(__dirname, './src/services'),
            hooks: path.resolve(__dirname, './src/hooks'),
            config: path.resolve(__dirname, './src/config'),
            constants: path.resolve(__dirname, './src/constants'),
            types: path.resolve(__dirname, './src/types'),
            utils: path.resolve(__dirname, './src/utils'),
            widgets: path.resolve(__dirname, './src/widgets'),
            router: path.resolve(__dirname, './src/router'),
        },
    },
});
