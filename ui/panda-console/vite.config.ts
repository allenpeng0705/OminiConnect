import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "node:path";

export default defineConfig({
  plugins: [react()],
  base: "/console/",
  build: {
    outDir: path.resolve(__dirname, "../../crates/panda-proxy/assets/console-ui"),
    emptyOutDir: true,
  },
});
