import { useLogin } from "~/composables/useLogin";

export default async function useLogout() {
  const { logout } = useLogin();
  await logout();
}
