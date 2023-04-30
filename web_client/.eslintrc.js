// eslint-disable-next-line @typescript-eslint/no-var-requires
const common = require("../.eslint-common");

module.exports = {
  ...common,
  plugins: ["react", "@typescript-eslint", "@next/eslint-plugin-next"],
};
