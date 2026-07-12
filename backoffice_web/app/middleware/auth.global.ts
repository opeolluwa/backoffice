import { useTokenStore } from "~/stores/token";

const publicRoutes = [
  "/",
  "/signup",
  "/forgotten-password",
  "/set-password",
  "/verify-otp",
  "/invitations/accept",
];

export default defineNuxtRouteMiddleware((to) => {
  const tokenStore = useTokenStore();

  if (publicRoutes.includes(to.path)) return;

  if (!tokenStore.isAccessTokenValid()) {
    return navigateTo("/");
  }
});
