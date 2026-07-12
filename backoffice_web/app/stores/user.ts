import { defineStore } from "pinia";
import type { UserProfile } from "~/bindings/UserProfile";
import axios from "axios";

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
    user: (state): UserProfile => ({
      identifier: state.identifier,
      firstName: state.firstName,
      lastName: state.lastName,
      email: state.email,
      // profilePicture: state.profilePicture,
      // username: state.username,
    }),
    fullName: (state) => `${state.firstName} ${state.lastName}`,
    userFirstName: (state) => state.firstName,
  },
  actions: {
    async initialize(token: string): Promise<UserProfile> {
      console.log({token})
      const userInformation = await this.fetchUserInformation(token);
      console.log({userInformation})
      this.$patch((state) => {
        state.identifier = userInformation.identifier;
        state.firstName = userInformation.firstName ?? "";
        state.lastName = userInformation.lastName ?? "";
        state.email = userInformation.email;
        // state.profilePicture = userInformation.profilePicture ?? "";
        // state.username = userInformation.username ?? "";
      });
      return userInformation;
    },
    async fetchUserInformation(token: string): Promise<UserProfile> {
      try {
        const response = await axios.get("/users/profile", {
          headers: { Authorization: `Bearer ${token}` },
        });
        console.log(response)
        return response.data as UserProfile;
      } catch (error) {
        throw new Error(`Failed to fetch user information due to ${error}`);
      }
    },
    setProfilePicture(url: string) {
      this.$patch({ profilePicture: url });
    },

    updateProfile(profile: UserProfile) {
      this.$patch({
        identifier: profile.identifier,
        firstName: profile.firstName ?? "",
        lastName: profile.lastName ?? "",
        email: profile.email,
        profilePicture: profile.profilePicture ?? "",
        username: profile.username ?? "",
      });
    },
  },
  persist: true,
});
