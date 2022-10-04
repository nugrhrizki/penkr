import { defineConfig } from "vite-plugin-windicss";
import scrollSnapPlugin from "windicss/plugin/scroll-snap";
import forms from "windicss/plugin/forms";

export default defineConfig({
  plugins: [scrollSnapPlugin, forms],
});
