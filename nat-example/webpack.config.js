const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const wasmExtensionRegExp = /\.wasm$/;
module.exports = {
  mode: "development",
  //...
  // resolve: {
  //   extensions: ['.wasm'],
  // },
  // module:{
  //   rules:[{
  //     test: wasmExtensionRegExp,
  //     include: path.resolve(__dirname,"s"),
  //     use: [{ loader: require.resolve('wasm-loader'), options: {} }]
  //   }]
  // },
  entry: './index.js',
  output: {
    path: path.resolve(__dirname, "public"),
    publicPath: "/assets/",
    filename: "bundle.js"
  },
  // static: { 
  //   directory: path.resolve(__dirname, './assets'), 
  //   publicPath: '/assets'
  // },
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
  },

  //plugins: [new HtmlWebpackPlugin()],
};