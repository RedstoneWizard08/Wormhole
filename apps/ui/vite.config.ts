import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
    clearScreen: false,
    plugins: [sveltekit()],

    server: {
        port: process.env.TAURI_WEB_DEV ? 4001 : 1420,
        strictPort: true,
        cors: true,

        proxy: {
            "^/__mr_cdn/.*": {
                target: "https://cdn.modrinth.com/",
                changeOrigin: true,
                rewrite: (path) => path.replace(/^\/__mr_cdn/, ""),
                followRedirects: true,
            },
        },

        hmr: process.env.TAURI_WEB_DEV
            ? {
                  clientPort: 443,
                  port: 4001,
                  protocol: "wss",
                  path: "/vite-hmr",
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
        target:
            process.env.TAURI_PLATFORM === "windows" ? "chrome105" : "safari13",
        minify: !process.env.TAURI_DEBUG ? "terser" : false,
        sourcemap: !!process.env.TAURI_DEBUG,

        terserOptions: {
            mangle: true,
        },
    },
});
