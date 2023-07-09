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
  ],
};
