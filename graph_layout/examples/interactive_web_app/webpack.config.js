const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

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
        module: {
            rules: [
                {
                    test: /\.s[ac]ss$/i,
                    use: ["style-loader", "css-loader", "sass-loader"],
                },
            ],
        },
        devServer: {
            contentBase: distPath,
            compress: argv.mode === "production",
            port: 8002,
            historyApiFallback: true,
        },
        entry: "./js/main.js",
        output: {
            path: distPath,
            filename: "app.js",
            webassemblyModuleFilename: "app.wasm",
            publicPath: "/",
        },
        plugins: [
            new CleanWebpackPlugin({ cleanStaleWebpackAssets: false }),
            new HtmlWebpackPlugin({
                title: "Graph Layout",
            }),
            new WasmPackPlugin({
                crateDirectory: ".",
                forceMode: "production",
                watchDirectories: ["../../src"],

            }),
        ],
        watch: argv.mode !== "production",
    };
};
