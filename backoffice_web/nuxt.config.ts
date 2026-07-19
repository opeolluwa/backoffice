// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from "@tailwindcss/vite";

export default defineNuxtConfig({
  compatibilityDate: "2025-07-15",
  devtools: { enabled: true },
  css: ["./app/assets/css/main.css"],
  alias: {
    "@bindings": "./app/bindings",
  },
  vite: {
    plugins: [tailwindcss()],
  },

  app: {
    pageTransition: { name: "slide-left", mode: "out-in" },
  },

  image: {
    imagekit: {
      baseURL: "https://ik.imagekit.io/vkqa6un9v",
    },
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
    "nuxt-seo-utils",
    "@nuxtjs/google-fonts",

    [
      "@nuxtjs/apollo",
      {
        clientConfigs: {
          default: "~/plugins/graphql.ts",
        },
      },
    ],
    [
      "@nuxtjs/google-fonts",
      {
        families: {
          Roboto: true,
          Inter: [400, 700],
          "Josefin+Sans": true,
          Lato: [100, 300],
          Raleway: {
            wght: [100, 400],
            ital: [100],
          },
          Poppins: "200..700",
          "Crimson Pro": {
            wght: "200..900",
            ital: "200..700",
          },
        },
      },
    ],
  ],
  routeRules: {
    "/api/**": {
      proxy: { to: "http://localhost:8000/api/**", bodyLimit: 26214400 },
    },
  },
  nitro: {
    devProxy: {
      "/api": {
        target: "http://localhost:8000/api",
        changeOrigin: true,
      },
    },
  },
  pinia: {
    storesDirs: ["./stores/**"],
  },
});
