import { useTokenStore } from "~/stores/token";
import { useUserInformationStore } from "~/stores/user";

export function useAuth() {
  const tokenStore = useTokenStore();
  const userStore = useUserInformationStore();

  async function restoreSession(): Promise<boolean> {
    if (!tokenStore.isAccessTokenValid()) {
      return false;
    }

    try {
      if (!userStore.identifier) {
        await userStore.initialize(tokenStore.accessToken);
      }
      return true;
    } catch {
      tokenStore.$reset();
      return false;
    }
  }

  function isAuthenticated(): boolean {
    return tokenStore.isAccessTokenValid();
  }

  function getToken(): string {
    return tokenStore.accessToken;
  }

  return {
    restoreSession,
    isAuthenticated,
    getToken,
  };
}
