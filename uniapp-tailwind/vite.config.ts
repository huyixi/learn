import { defineConfig } from "vite";
import uni from "@dcloudio/vite-plugin-uni";
import postcss from './postcss.config.ts'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [uni()],
  css: {
    postcss
  }
});
