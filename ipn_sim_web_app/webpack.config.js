const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const ExtraWatchWebpackPlugin = require("extra-watch-webpack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    devtool: "inline-source-map",
    resolve: {
      alias: {
        "~": __dirname,
      },
      extensions: [".ts", ".js"],
    },
    devServer: {
      contentBase: distPath,
      compress: argv.mode === "production",
      port: 8000,
    },
    entry: "./js/main.js",
    output: {
      path: distPath,
      filename: "app.js",
      webassemblyModuleFilename: "app.wasm",
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: ["style-loader", "css-loader", "sass-loader"],
        },
        {
          test: /\.ts?$/,
          use: "ts-loader",
          exclude: /node_modules/,
        },
      ],
    },
    plugins: [
      new CleanWebpackPlugin({ cleanStaleWebpackAssets: false }),
      new CopyWebpackPlugin({
        patterns: [{ from: "static", to: "." }],
      }),
      new HtmlWebpackPlugin({
        title: "IPN Sim",
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        // forceMode: 'production'
      }),
      new ExtraWatchWebpackPlugin({
        dirs: ["./ts", "../ipn_sim_lib"],
      }),
    ],
    watch: argv.mode !== "production",
  };
};
