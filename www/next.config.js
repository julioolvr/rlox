module.exports = {
  exportPathMap: function() {
    return {
      "/": { page: "/" }
    };
  },
  webpack: config => {
    config.module.rules.push({
      test: /\.rs$/,
      use: [
        {
          loader: "wasm-loader"
        },
        {
          loader: "rust-native-wasm-loader",
          options: {
            release: true,
            gc: true
          }
        }
      ]
    });

    return config;
  }
};
