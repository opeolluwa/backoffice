import type { UserProfile } from "~/bindings/UserDto";

export function useGetInitials(user: UserProfile) {
  const name = `${user.firstName} ${user.lastName}`;  
  return name
    .split(" ")
    .map((n) => n[0])
    .join("")
    .toUpperCase()
    .slice(0, 2);
}