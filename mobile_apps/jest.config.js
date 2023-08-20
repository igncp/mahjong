module.exports = {
  moduleNameMapper: {
    "\\.(css|less)$": "<rootDir>/src/mocks/file-mock.ts",
    "\\.(jpg|ico|jpeg|png|gif|eot|otf|webp|svg|ttf|woff|woff2|mp4|webm|wav|mp3|m4a|aac|oga)$":
      "<rootDir>/src/mocks/file-mock.ts",
  },
  transformIgnorePatterns: [
    "/node_modules/(?!(mahjong_sdk|@react-native|react-native)/)",
  ],
};
