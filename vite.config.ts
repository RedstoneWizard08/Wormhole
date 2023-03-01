import path from "path";
import { defineConfig } from "vite";
import { preact } from "@preact/preset-vite";

export default defineConfig({
    clearScreen: false,

    plugins: [preact()],

    server: {
        port: process.env.TAURI_WEB_DEV ? 4000 : 1420,
        strictPort: true,
        cors: true,

        proxy: {
            "^/_spacedock/.*": {
                target: "https://spacedock.info/api",
                changeOrigin: true,
                rewrite: (path) => path.replace(/^\/_spacedock/, ""),
            }
        },

        hmr: process.env.TAURI_WEB_DEV ? {
            clientPort: 443,
            port: 4000,
            protocol: "wss",
        } : {},
    },

    root: path.join(__dirname, "app"),
    envPrefix: ["VITE_", "TAURI_"],

    build: {
        target:
            process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",

        minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
        sourcemap: !!process.env.TAURI_DEBUG,
    },
});
