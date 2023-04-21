module.exports = {
    env: {
        browser: true,
        es2017: true,
        node: true,
    },

    extends: ["eslint:recommended", "plugin:@typescript-eslint/recommended"],

    parser: "@typescript-eslint/parser",
    plugins: ["svelte3", "@typescript-eslint"],
    ignorePatterns: ["*.cjs"],
    overrides: [{ files: ["*.svelte"], processor: "svelte3/svelte3" }],

    settings: {
        "svelte3/typescript": () => require("typescript"),
    },

    parserOptions: {
        ecmaVersion: 2020,
        sourceType: "module",
    },

    rules: {
        "no-case-declarations": "off",
        "@typescript-eslint/no-explicit-any": "off",
        "@typescript-eslint/triple-slash-reference": "off",
        "@typescript-eslint/no-non-null-assertion": "off",
    },
};
