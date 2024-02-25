// eslint-disable-next-line @typescript-eslint/no-var-requires
const common = require("../.eslint-common");

module.exports = {
  ...common,
  extends: [...common.extends, "plugin:playwright/recommended"],
  plugins: [
    "react",
    "@typescript-eslint",
    "@next/eslint-plugin-next",
    "playwright",
    "perfectionist",
  ],
  rules: {
    ...common.rules,

    "perfectionist/sort-array-includes": "error",
    "perfectionist/sort-classes": "error",
    "perfectionist/sort-enums": "error",
    "perfectionist/sort-exports": "error",
    "perfectionist/sort-interfaces": "error",
    "perfectionist/sort-jsx-props": "error",
    "perfectionist/sort-maps": "error",
    "perfectionist/sort-named-exports": "error",
    "perfectionist/sort-object-types": "error",
    "perfectionist/sort-objects": "error",
    "perfectionist/sort-union-types": "error",
  },
};
