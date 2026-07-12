import { useTokenStore } from "~/stores/token";
import { useUserInformationStore } from "~/stores/user";
import api from "~/plugin/api";

interface LoginCredentials {
  email: string;
  password: string;
}

interface LoginResult {
  success: boolean;
  error?: string;
}

export function useLogin() {
  const tokenStore = useTokenStore();
  const userStore = useUserInformationStore();
  const router = useRouter();

  async function login(credentials: LoginCredentials): Promise<LoginResult> {
    try {
      const { status, data: respData } = await api.post("/login", credentials);

      if (status !== 200) {
        return {
          success: false,
          error: respData?.message || "Login failed",
        };
      }

      tokenStore.persistAccessToken(respData.data.token);

      try {
        await userStore.initialize(respData.data.token);
      } catch(error) {
        // User profile fetch failed, but login was successful
        console.error(error);
      }

      await router.push("/home");
      return { success: true };
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } catch (err: any) {
      return {
        success: false,
        error: err.message || "An error occurred. Please try again.",
      };
    }
  }

  async function logout() {
    tokenStore.$reset();
    userStore.$reset();
    await router.push("/");
  }

  function isAuthenticated(): boolean {
    return tokenStore.isAccessTokenValid();
  }

  function getToken(): string {
    return tokenStore.accessToken;
  }

  return {
    login,
    logout,
    isAuthenticated,
    getToken,
  };
}
