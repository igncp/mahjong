const disabledRules = {
  "@typescript-eslint/no-empty-function": 0,
  "@typescript-eslint/no-extra-semi": 0,
  "comma-dangle": 0,
  "react/prop-types": 0,
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
  plugins: ["react", "@typescript-eslint"],
  rules: {
    ...disabledRules,

    "@typescript-eslint/member-ordering": 2,
    "arrow-body-style": 2,
    "no-else-return": 2,
    "object-shorthand": 2,
    "prefer-destructuring": 2,
    "prefer-template": 2,
    "react/jsx-sort-props": 2,
    "sort-keys": [2, "asc", { caseSensitive: false, natural: true }],
    "spaced-comment": 2,
  },
  settings: {
    react: {
      version: "detect",
    },
  },
};
