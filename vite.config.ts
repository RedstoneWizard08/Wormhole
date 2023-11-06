import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
    clearScreen: false,

    plugins: [sveltekit()],

    server: {
        port: process.env.TAURI_WEB_DEV ? 4000 : 1420,
        strictPort: true,
        cors: true,

        proxy: {
            "^/_spacedock/.*": {
                target: "https://spacedock.info/",
                changeOrigin: true,
                rewrite: (path) => path.replace(/^\/_spacedock/, ""),
                followRedirects: true,
            },
        },

        hmr: process.env.TAURI_WEB_DEV
            ? {
                  clientPort: 443,
                  port: 4000,
                  protocol: "wss",
              }
            : {},
    },

    envPrefix: ["VITE_", "TAURI_"],

    resolve: {
        alias: {
            path: "path-browserify",
        },
    },

    build: {
        target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",

        minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
        sourcemap: !!process.env.TAURI_DEBUG,
    },
});
