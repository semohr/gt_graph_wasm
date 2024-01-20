const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin(),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "."),
        }),
        new CopyWebpackPlugin({
            patterns: [
                { from: 'test_data', to: 'test_data' }
            ]
        })
    ],
    mode: 'development',
    experiments: {
        asyncWebAssembly: true
    }
};