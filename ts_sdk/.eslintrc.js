// eslint-disable-next-line @typescript-eslint/no-var-requires
const common = require("../.eslint-common");

module.exports = {
  ...common,
  root: true,
  rules: {
    ...common.rules,
    quotes: 0,
  },
};
