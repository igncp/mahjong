const nextConfig = {
  output: "export",
  webpack: (config) => {
    config.experiments = { ...config.experiments, asyncWebAssembly: true };

    return config;
  },
};

module.exports = nextConfig;
