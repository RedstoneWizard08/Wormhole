// @ts-check

import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import("@sveltejs/kit").Config} */
export default {
    preprocess: vitePreprocess(),

    kit: {
        adapter: adapter({
            fallback: "index.html",

            // eslint-disable-next-line no-undef
            pages: process.env.TAURI_WEB_DEV ? "web-build" : "build",
        }),

        alias: {
            $components: "src/components",
            $assets: "src/assets",
            $api: "src/api",
            $lib: "src/lib",
            $routes: "src/routes",
            $styles: "src/styles",
            $bindings: "src/api/bindings/app.ts",
        },
    },
};
