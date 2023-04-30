const nextConfig = {
  output: "export",
  transpilePackages: ["mahjong_sdk"],
  webpack: (config) => {
    config.experiments = { ...config.experiments, asyncWebAssembly: true };

    return config;
  },
};

module.exports = nextConfig;
