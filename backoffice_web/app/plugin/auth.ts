import { useAuth } from "~/composables/useAuth";

export default defineNuxtPlugin(async () => {
  const { restoreSession } = useAuth();
  await restoreSession();
});
