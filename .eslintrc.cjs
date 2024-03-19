module.exports = {
    env: {
        browser: true,
        es2017: true,
        node: true,
    },

    extends: ["eslint:recommended", "plugin:svelte/recommended"],

    parser: "@typescript-eslint/parser",
    plugins: ["svelte", "@typescript-eslint"],
    ignorePatterns: ["*.cjs"],
    overrides: [
        {
            files: ["*.svelte"],
            parser: "svelte-eslint-parser",
            parserOptions: {
                parser: "@typescript-eslint/parser",
            },
        },
    ],

    parserOptions: {
        ecmaVersion: "next",
        sourceType: "module",
        extraFileExtensions: [".svelte"],
    },

    rules: {
        "no-case-declarations": "off",
        "@typescript-eslint/no-explicit-any": "off",
        "@typescript-eslint/triple-slash-reference": "off",
        "@typescript-eslint/no-non-null-assertion": "off",
        "svelte/valid-compile": "off",
        "svelte/no-at-html-tags": "off",
        "no-unused-vars": "off",
    },
};
