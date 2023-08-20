module.exports = {
  extends: [
    "stylelint-config-standard-scss",
    // https://github.com/kutsan/stylelint-config-clean-order
    "stylelint-config-clean-order",
  ],
  plugins: ["stylelint-scss"],
  rules: {
    "alpha-value-notation": null,
    "color-function-notation": null,
    "color-no-invalid-hex": true,
    "no-descending-specificity": true,
    "scss/dollar-variable-empty-line-before": null,
    "selector-class-pattern": null,
    "selector-pseudo-class-no-unknown": null, // Error with `:global` in CSS Modules
  },
};
