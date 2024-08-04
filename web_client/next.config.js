// eslint-disable-next-line @typescript-eslint/no-var-requires
const webpack = require("webpack");

const nextConfig = {
  output: "export",
  webpack: (config, { isServer }) => {
    config.experiments = { ...config.experiments, asyncWebAssembly: true };

    if (isServer) {
      config.plugins.push(
        new webpack.NormalModuleReplacementPlugin(/pkg$/, "src/pkg_mock.js"),
      );
    }

    return config;
  },
};

module.exports = nextConfig;
