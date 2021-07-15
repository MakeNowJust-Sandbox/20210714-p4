const path = require("path");

const CopyPlugin = require("copy-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    main: "./scripts/main.tsx",
  },
  output: {
    path: dist,
    filename: "[name].js",
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        include: path.resolve(__dirname, "scripts"),
        use: {
          loader: "@sucrase/webpack-loader",
          options: {
            transforms: ["jsx"],
          },
        },
      },
      {
        test: /\.ts$/,
        include: path.resolve(__dirname, "scripts"),
        use: {
          loader: "@sucrase/webpack-loader",
          options: {
            transforms: ["typescript"],
          },
        },
      },
      {
        test: /\.tsx$/,
        include: path.resolve(__dirname, "scripts"),
        use: {
          loader: "@sucrase/webpack-loader",
          options: {
            transforms: ["jsx", "typescript"],
          },
        },
      },
      {
        test: /\.s[ac]ss$/,
        include: [
          path.resolve(__dirname, "scripts"),
          path.resolve(__dirname, "node_modules", "bulma"),
        ],
        use: [MiniCssExtractPlugin.loader, "css-loader", "sass-loader"],
      },
      {
        test: /\.css/,
        include: [path.resolve(__dirname, "scripts")],
        use: [MiniCssExtractPlugin.loader, "css-loader"],
      },
    ],
  },
  resolve: {
    extensions: [".js", ".json", ".ts", ".tsx", ".wasm"],
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        {
          from: path.resolve(__dirname, "static"),
          to: dist,
        },
      ],
    }),
    new MiniCssExtractPlugin(),
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};
