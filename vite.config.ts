import { defineConfig } from "vite-plus";

export default defineConfig({
  fmt: {
    endOfLine: process.platform === "win32" ? "crlf" : "lf",
  },
  lint: {
    jsPlugins: [{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" }],
    rules: { "vite-plus/prefer-vite-plus-imports": "error" },
    options: { typeAware: true, typeCheck: true },
  },
  run: {
    cache: true,
  },
});
