// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
        target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",

        minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
        sourcemap: !!process.env.TAURI_DEBUG,
    },
});
