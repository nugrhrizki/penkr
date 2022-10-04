import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import WindiCSS from "vite-plugin-windicss";
import Icons from "unplugin-icons/vite";

export default defineConfig({
  plugins: [solidPlugin(), WindiCSS(), Icons({ compiler: "solid" })],
  server: {
    port: 3000,
  },
  build: {
    target: "esnext",
  },
});
