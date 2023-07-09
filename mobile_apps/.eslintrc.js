// eslint-disable-next-line @typescript-eslint/no-var-requires
const common = require("../.eslint-common");

module.exports = {
  ...common,
  extends: [
    "eslint:recommended",
    "prettier",
    "plugin:react/recommended",
    "plugin:@typescript-eslint/recommended",
    "@react-native-community",
  ],
  plugins: [...common.plugins, "jest"],
  root: true,
  rules: {
    ...common.rules,
    quotes: 0,
  },
};
