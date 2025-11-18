// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from "@tailwindcss/vite";

export default defineNuxtConfig({
  compatibilityDate: "2025-07-15",
  devtools: { enabled: true },
  css: ["./app/assets/css/main.css"],
  vite: {
    plugins: [tailwindcss()],
  },

  app: {
    pageTransition: { name: "slide-left", mode: "out-in" },
  },
  colorMode: {
    preference: "light",
    fallback: "light",
    classSuffix: "",
  },
  modules: [
    "@nuxt/image",
    "@nuxt/scripts",
    "@nuxt/ui",
    "@nuxt/test-utils",
    "@nuxt/eslint",
    "@pinia/nuxt",
    "nuxt-viewport",
    "@nuxtjs/color-mode",
    "pinia-plugin-persistedstate/nuxt",
    "@vueuse/nuxt",
  ],
  pinia: {
    storesDirs: ["./stores/**"],
  },
});
