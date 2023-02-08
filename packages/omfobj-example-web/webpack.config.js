const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: './src/index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js',
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './src/index.html',
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../../crates/omfobj-viewer-js")
    }),
  ],
  mode: 'development',
  experiments: {
    asyncWebAssembly: true,
  },
};
