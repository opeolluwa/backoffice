// https://nuxt.com/docs/api/configuration/nuxt-config
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const rootDir = fileURLToPath(new URL(".", import.meta.url));

export default defineNuxtConfig({
  compatibilityDate: "2025-07-15",
  devtools: { enabled: true },
  ssr: true,
  css: ["./app/assets/css/main.css"],
  modules: [
    "@nuxt/eslint",
    "@nuxt/hints",
    "@nuxt/ui",
    "@nuxt/image",
    "nuxt-viewport",
    // "nuxt-notify",
  ],
  vite: {
    build: {
      outDir: resolve(rootDir, "../backoffice_console/assets"),
      // publicDir: resolve(rootDir, "../backoffice_console/assets/public"),
    },
  },
});
