import { defineStore } from "pinia";
import type { UserProfile } from "~/bindings/UserProfile";
import api from "~/plugin/api";

export const useUserInformationStore = defineStore("user_information", {
  state: () => ({
    identifier: "",
    firstName: "",
    lastName: "",
    email: "",
    profilePicture: "",
    username: "",
  }),

  getters: {
    user: (
      state,
    ): UserProfile & {
      fullName: string;
      profilePicture: string;
      username: string;
    } => ({
      identifier: state.identifier,
      firstName: state.firstName,
      lastName: state.lastName,
      email: state.email,
      profilePicture: state.profilePicture,
      username: state.username,
      fullName: `${state.firstName} ${state.lastName}`,
    }),
    fullName: (state) => `${state.firstName} ${state.lastName}`,
    userFirstName: (state) => state.firstName,
  },
  actions: {
    async initialize(token: string): Promise<UserProfile> {
      const userInformation = await this.fetchUserInformation(token);
      this.$patch((state) => {
        state.identifier = userInformation.identifier;
        state.firstName = userInformation.firstName ?? "";
        state.lastName = userInformation.lastName ?? "";
        state.email = userInformation.email;
        const info = userInformation as Record<string, unknown>;
        state.profilePicture = (info.profilePicture as string) ?? "";
        state.username = (info.username as string) ?? "";
      });
      return userInformation;
    },
    async fetchUserInformation(token: string): Promise<UserProfile> {
      try {
        const response = await api.get("/users/profile", {
          headers: { Authorization: `Bearer ${token}` },
        });

        return response.data.data as UserProfile;
      } catch (error) {
        throw new Error(`Failed to fetch user information due to ${error}`);
      }
    },
    setProfilePicture(url: string) {
      this.$patch({ profilePicture: url });
    },

    async updateProfile(profile: {
      firstName: string;
      lastName: string;
      username?: string;
    }) {
      const response = await api.put("/users/profile", {
        firstName: profile.firstName,
        lastName: profile.lastName,
        username: profile.username ?? null,
      });
      const data = response.data.data as Record<string, unknown>;
      this.$patch({
        identifier: data.identifier as string,
        firstName: (data.firstName as string) ?? "",
        lastName: (data.lastName as string) ?? "",
        email: data.email as string,
        profilePicture: (data.profilePicture as string) ?? "",
        username: (data.username as string) ?? "",
      });
    },

    async changePassword(payload: {
      currentPassword: string;
      newPassword: string;
      confirmPassword: string;
    }) {
      await api.post("/users/change-password", {
        currentPassword: payload.currentPassword,
        newPassword: payload.newPassword,
        confirmPassword: payload.confirmPassword,
      });
    },

    async uploadProfilePicture(file: File) {
      const formData = new FormData();
      formData.append("file", file);
      const response = await api.post("/users/profile-picture", formData, {
        headers: { "Content-Type": "multipart/form-data" },
      });
      const data = response.data.data as Record<string, unknown>;
      this.$patch({
        profilePicture: (data.profilePicture as string) ?? "",
      });
    },
  },
  persist: true,
});
