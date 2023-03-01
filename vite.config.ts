import path from "path";
import { defineConfig } from "vite";
import { preact } from "@preact/preset-vite";

export default defineConfig({
    clearScreen: false,

    plugins: [preact()],

    server: {
        port: 1420,
        strictPort: true,
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
