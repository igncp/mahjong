module.exports = {
  preset: "react-native",
  setupFiles: ["./src/tests/setup.ts"],
  transformIgnorePatterns: [
    "/node_modules/(?!(mahjong_sdk|@react-native|react-native)/)",
  ],
};
