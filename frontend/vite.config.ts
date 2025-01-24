import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { resolve } from 'path';
import { type UserConfig } from 'vite';

// https://vitejs.dev/config/
const config: UserConfig = {
    plugins: [react()],
    server: {
        port: 42068,
    },
    // https://medium.com/@pushplaybang/absolutely-dont-use-relative-paths-imports-in-your-vite-react-project-c8593f93bbea
    resolve: {
        alias: {
            src: '/src',
            tests: '/tests',
        },
    },
    build: {
        rollupOptions: {
            input: {
                index: resolve(__dirname, 'index.html'),
                mini_game: resolve(__dirname, 'mini-game/index.html'),
            },
        },
    },
};

if (process.env.NODE_ENV === 'development') {
    // not sure how to fix this ts error, but the line itself is fine
    // @ts-ignore
    // config.build.rollupOptions.input.development = resolve(
    //     __dirname,
    //     'development/index.html',
    // );
}

export default defineConfig(config);
