const disabledRules = {
  "@typescript-eslint/no-empty-function": 0,
  "@typescript-eslint/no-extra-semi": 0,
  "react/react-in-jsx-scope": 0,
};

module.exports = {
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  extends: [
    "eslint:recommended",
    "prettier",
    "plugin:react/recommended",
    "plugin:@typescript-eslint/recommended",
  ],
  overrides: [],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    ecmaVersion: "latest",
    sourceType: "module",
  },
  plugins: ["react", "@typescript-eslint", "@next/eslint-plugin-next"],
  rules: {
    "@typescript-eslint/member-ordering": 2,
    "arrow-body-style": 2,
    "no-else-return": 2,
    "prefer-template": 2,
    "react/jsx-sort-props": 2,
    "sort-keys": [2, "asc", { caseSensitive: false, natural: true }],

    ...disabledRules,
  },
  settings: {
    react: {
      version: "detect",
    },
  },
};
