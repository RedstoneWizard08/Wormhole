import globals from "globals";
import svelte from "eslint-plugin-svelte";
import svelteParser from "svelte-eslint-parser";
import eslint from "@eslint/js";
import tslint from "typescript-eslint";
import pluginImport from "eslint-plugin-import";
import jsdoc from "eslint-plugin-jsdoc";
import markdown from "eslint-plugin-markdown";

const ignores = [
    "*.cjs",
    "node_modules",
    ".svelte-kit",
    "build",
    "*.md",
    "LICENSE",
    "*.png",
    "*.svg",
    "*.gif",
    "*.jpg",
    "*.jpeg",
    "*.jfif",
    "*.lock",
    "pnpm-lock.yaml",
    "package-lock.json",
    ".git",
    "*.rs",
    "target",
    "web-build",
];

export default tslint.config(
    { ignores },

    ...markdown.configs.recommended,
    eslint.configs.recommended,
    ...tslint.configs.recommended,
    ...svelte.configs["flat/recommended"],
    jsdoc.configs["flat/recommended"],

    {
        files: ["**/*.svelte"],

        languageOptions: {
            parser: svelteParser,

            parserOptions: {
                parser: tslint.parser,
            },

            globals: {
                ...globals.browser,
            },
        },
    },

    {
        files: ["**/*.ts"],

        languageOptions: {
            parser: tslint.parser,
        },
    },

    {
        plugins: {
            "@typescript-eslint": tslint.plugin,
            import: pluginImport,
        },

        rules: {
            semi: "warn",
            "svelte/sort-attributes": "warn",
            "no-case-declarations": "off",
            "@typescript-eslint/no-explicit-any": "off",
            "@typescript-eslint/no-unused-vars": "off",
            "@typescript-eslint/triple-slash-reference": "off",
            "@typescript-eslint/no-non-null-assertion": "off",
            "svelte/valid-compile": "off",
            "svelte/no-at-html-tags": "off",
            "no-unused-vars": "off",
            "jsdoc/require-param": "off",
            "jsdoc/require-returns": "off",
            "jsdoc/require-jsdoc": "off",
        },
    },

    {
        files: ["src/api/bindings/**/*.ts"],

        rules: {
            semi: "off",
            "jsdoc/no-multi-asterisks": "off",
        },
    },
);
