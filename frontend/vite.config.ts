import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import glsl from "vite-plugin-glsl";
import { resolve } from "path";

export default defineConfig({
  plugins: [
    react(),
    glsl({
      include: [
        "**/*.glsl",
        "**/*.wgsl",
        "**/*.vert",
        "**/*.frag",
        "**/*.vs",
        "**/*.fs",
      ],
      exclude: undefined,
      warnDuplicatedImports: true,
      removeDuplicatedImports: true,
      defaultExtension: "glsl",
      root: resolve(__dirname, "src"),
    }),
  ],
  resolve: {
    alias: {
      "@": resolve(__dirname, "src"),
      "@components": resolve(__dirname, "src/components"),
      "@assets": resolve(__dirname, "src/assets"),
      "@styles": resolve(__dirname, "src/styles"),
      "@utils": resolve(__dirname, "src/utils"),
      "@types": resolve(__dirname, "src/types"),
      "@hooks": resolve(__dirname, "src/hooks"),
    },
  },
});
