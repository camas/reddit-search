const path = require('path')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')

const distPath = path.resolve(__dirname, 'dist')
module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000
    },
    entry: './bootstrap.js',
    output: {
      // publicPath: '/reddit-search/',
      path: distPath,
      filename: 'reddit-search.js',
      webassemblyModuleFilename: 'reddit-search.wasm'
    },
    plugins: [
      new WasmPackPlugin({
        crateDirectory: '.',
        extraArgs: '--no-typescript'
      }),
      new HtmlWebpackPlugin({
        title: 'Reddit Search'

      })
    ],
    watch: argv.mode !== 'production',
    module: {
      rules: [
        {
          test: /\.css$/,
          exclude: /node_modules/,
          use: [
            {
              loader: 'style-loader'
            },
            {
              loader: 'css-loader',
              options: {
                importLoaders: 1
              }
            },
            {
              loader: 'postcss-loader'
            }
          ]
        }
      ]
    }
  }
}
