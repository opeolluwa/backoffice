import { defineStore } from "pinia";
import api from "~/plugin/api";
import type { TeamsInterface as TeamMember } from "~/bindings/TeamsInterface";

const useTeamsStore = defineStore("teams", {
  state: () => ({
    members: [] as Array<TeamMember>,
    currentMember: null as TeamMember | null,
    memberCount: 0,
  }),

  actions: {
    async createTeamMember(payload: {
      name: string;
      email: string;
      phone?: string | null;
      role?: string | null;
    }) {
      const res = await api.post("/teams", payload);
      const created = res.data?.data as TeamMember;
      this.members.unshift(created);
      this.memberCount++;
      return created;
    },

    async fetchAllMembers() {
      const res = await api.get("/teams");
      this.members = (res.data?.data as Array<TeamMember>) || [];
    },

    async countMembers() {
      const res = await api.get("/teams/count");
      this.memberCount = (res.data?.data as number) || 0;
      return this.memberCount;
    },

    async fetchMemberByIdentifier(identifier: string) {
      const res = await api.get(`/teams/${identifier}`);
      this.currentMember = (res.data?.data as TeamMember) || null;
      return this.currentMember;
    },

    async updateMember(
      identifier: string,
      payload: {
        name?: string;
        phone?: string | null;
        role?: string | null;
        blocked?: boolean | null;
      },
    ) {
      const res = await api.put(`/teams/${identifier}`, payload);
      const updated = res.data?.data as TeamMember;
      const idx = this.members.findIndex((m) => m.identifier === identifier);
      if (idx !== -1) this.members[idx] = updated;
      if (this.currentMember?.identifier === identifier)
        this.currentMember = updated;
      return updated;
    },

    async deleteMember(identifier: string) {
      await api.delete(`/teams/${identifier}`);
      this.members = this.members.filter((m) => m.identifier !== identifier);
      this.memberCount = Math.max(0, this.memberCount - 1);
      if (this.currentMember?.identifier === identifier)
        this.currentMember = null;
    },

    async blockMember(identifier: string) {
      const res = await api.put(`/teams/${identifier}/block`);
      const updated = res.data?.data as TeamMember;
      const idx = this.members.findIndex((m) => m.identifier === identifier);
      if (idx !== -1) this.members[idx] = updated;
      if (this.currentMember?.identifier === identifier)
        this.currentMember = updated;
      return updated;
    },

    async unblockMember(identifier: string) {
      const res = await api.put(`/teams/${identifier}/unblock`);
      const updated = res.data?.data as TeamMember;
      const idx = this.members.findIndex((m) => m.identifier === identifier);
      if (idx !== -1) this.members[idx] = updated;
      if (this.currentMember?.identifier === identifier)
        this.currentMember = updated;
      return updated;
    },
  },
});

export { useTeamsStore };
