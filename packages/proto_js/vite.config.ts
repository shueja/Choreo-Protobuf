import dts from "vite-plugin-dts";
import path from "path";
import { defineConfig, UserConfig } from "vite";

export default defineConfig({
  base: "./",
  plugins: [dts({ rollupTypes: true, insertTypesEntry: true })],
  build: {
    emptyOutDir: true,
    sourcemap: true,
    lib: {
      entry: path.resolve(__dirname, "src/index.ts"),
      formats: ["es"], // ES/CJS for npm, IIFE for CDN
      fileName: (format) =>
        `index.${format === "es" ? "mjs" : format === "cjs" ? "cjs" : "iife.js"}`,
    },
  },
} satisfies UserConfig);