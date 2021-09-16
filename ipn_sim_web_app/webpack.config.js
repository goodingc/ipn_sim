const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const {CleanWebpackPlugin} = require("clean-webpack-plugin");
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
        watchOptions: {
            aggregateTimeout: 1000,
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
                {
                    test: /\.woff(2)?(\?v=[0-9]\.[0-9]\.[0-9])?$/,
                    include: path.resolve(
                        __dirname,
                        "../node_modules/bootstrap-icons/font/fonts"
                    ),
                    use: {
                        loader: "file-loader",
                        options: {
                            name: "[get_name].[ext]",
                            outputPath: "webfonts",
                            publicPath: "../webfonts",
                        },
                    },
                },
                // {
                //   test: /\.json$/,
                //   loader: "json-loader",
                // },
            ],
        },
        plugins: [
            new CleanWebpackPlugin({cleanStaleWebpackAssets: false}),
            new CopyWebpackPlugin({
                patterns: [{from: "static", to: "."}],
            }),
            new HtmlWebpackPlugin({
                title: "IPN Sim",
            }),
            new WasmPackPlugin({
                crateDirectory: ".",
                forceMode: "production",
                watchDirectories: [
                    "../ipn_sim_lib",
                    "../ipn_sim_reports",
                    "../graph_layout"
                ],
            }),
            new ExtraWatchWebpackPlugin({
                dirs: ["ts"],
            }),
        ],
        watch: argv.mode !== "production",
    };
};
