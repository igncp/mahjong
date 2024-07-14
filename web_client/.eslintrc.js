// eslint-disable-next-line @typescript-eslint/no-var-requires
const common = require("../.eslint-common");

const paddingRule = [
  "error",
  { blankLine: "always", next: "return", prev: "*" },
]
  .concat(
    [
      "const",
      "if",
      "interface",
      "multiline-block-like",
      "multiline-const",
      "multiline-expression",
      "type",
    ]
      .map((item) => [
        { blankLine: "always", next: "*", prev: item },
        { blankLine: "always", next: item, prev: "*" },
      ])
      .flat()
  )
  .concat([
    {
      blankLine: "any",
      next: ["singleline-const"],
      prev: ["singleline-const"],
    },
  ]);

module.exports = {
  ...common,
  extends: [...common.extends, "plugin:playwright/recommended"],
  plugins: [
    "@next/eslint-plugin-next",
    "@stylistic",
    "@typescript-eslint",
    "import",
    "perfectionist",
    "playwright",
    "react",
    "react-hooks",
  ],
  rules: {
    ...common.rules,

    "@stylistic/padding-line-between-statements": paddingRule,

    "@typescript-eslint/consistent-type-imports": "error",
    "@typescript-eslint/no-shadow": "error",
    "@typescript-eslint/no-unused-vars": "error",
    "@typescript-eslint/no-use-before-define": "error",

    "import/no-duplicates": "error",

    "newline-before-return": 2,
    "no-console": ["error", { allow: ["warn", "error"] }],
    "no-else-return": "error",
    "no-extra-semi": "off",
    "no-shadow": "off",
    "no-unused-vars": "off",
    "no-useless-return": "error",
    "object-shorthand": "error",

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

    "prefer-const": "error",
    "prefer-destructuring": ["error", { array: false, object: true }],
    "prefer-spread": "error",
    "prefer-template": "error",
    "quote-props": ["error", "consistent-as-needed"],

    "react-hooks/exhaustive-deps": "error",
    "react-hooks/rules-of-hooks": "error",

    "react/destructuring-assignment": [
      "error",
      "always",
      { destructureInSignature: "always" },
    ],
    "react/display-name": "off",
    "react/function-component-definition": "off",
    "react/jsx-boolean-value": "error",
    "react/jsx-curly-brace-presence": "error",
    "react/jsx-filename-extension": "off",
    "react/jsx-fragments": "error",
    "react/jsx-key": ["error", { warnOnDuplicates: true }],
    "react/jsx-no-target-blank": "off",
    "react/jsx-no-useless-fragment": "error",
    "react/jsx-one-expression-per-line": "off",
    "react/jsx-props-no-spreading": "off",
    "react/jsx-sort-props": "error",
    "react/no-array-index-key": "off",
    "react/no-unescaped-entities": "off",
    "react/no-unknown-property": "off",
    "react/no-unstable-nested-components": "error",
    "react/no-unused-prop-types": "error",
    "react/prop-types": "off",
    "react/react-in-jsx-scope": "off",
    "react/require-default-props": "off",
    "react/self-closing-comp": "error",
  },
};
