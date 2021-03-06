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
      port: 8001,
      historyApiFallback: true,
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
        // {
        //   test: /\.woff(2)?(\?v=[0-9]\.[0-9]\.[0-9])?$/,
        //   include: path.resolve(
        //     __dirname,
        //     "../node_modules/bootstrap-icons/font/fonts"
        //   ),
        //   use: {
        //     loader: "file-loader",
        //     options: {
        //       name: "[get_name].[ext]",
        //       outputPath: "webfonts",
        //       publicPath: "../webfonts",
        //     },
        //   },
        // },
        {
          test: /\.(png|woff|woff2|eot|ttf|svg)$/,
          loader: "url-loader",
          options: {
            limit: 100000,
          },
        },
        // {
        //   test: /\.json$/,
        //   loader: "json-loader",
        // },
      ],
    },
    plugins: [
      new CleanWebpackPlugin({ cleanStaleWebpackAssets: false }),
      // new CopyWebpackPlugin({
      //   patterns: [{ from: "static", to: "." }],
      // }),
      new HtmlWebpackPlugin({
        title: "IPN Sim Router Zoo",
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        forceMode: "production",
        watchDirectories: ["../ipn_sim_lib", "../ipn_sim_reports"],
      }),
      // new ExtraWatchWebpackPlugin({
      //   dirs: ["ts"],
      // }),
    ],
    watch: argv.mode !== "production",
    experiments: {
      asyncWebAssembly: true,
    },
  };
};
