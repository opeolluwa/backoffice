import { defineStore } from "pinia";
import api from "~/plugin/api";
import type { InvitationInterface as Invitation } from "~/bindings/InvitationInterface";

const useInvitationsStore = defineStore("invitations", {
  state: () => ({
    invitations: [] as Array<Invitation>,
    currentInvitation: null as Invitation | null,
    invitationCount: 0,
  }),

  actions: {
    async createInvitation(email: string) {
      const res = await api.post("/invitations", { email });
      const created = res.data?.data as Invitation;
      this.invitations.unshift(created);
      this.invitationCount++;
      return created;
    },

    async fetchAllInvitations() {
      const res = await api.get("/invitations");
      this.invitations = (res.data?.data as Array<Invitation>) || [];
    },

    async countInvitations() {
      const res = await api.get("/invitations/count");
      this.invitationCount = (res.data?.data as number) || 0;
      return this.invitationCount;
    },

    async fetchInvitationByIdentifier(identifier: string) {
      const res = await api.get(`/invitations/${identifier}`);
      this.currentInvitation = (res.data?.data as Invitation) || null;
      return this.currentInvitation;
    },

    async blockInvitation(identifier: string) {
      const res = await api.put(`/invitations/${identifier}/block`);
      const updated = res.data?.data as Invitation;
      const idx = this.invitations.findIndex(
        (i) => i.identifier === identifier,
      );
      if (idx !== -1) this.invitations[idx] = updated;
      if (this.currentInvitation?.identifier === identifier)
        this.currentInvitation = updated;
      return updated;
    },

    async deleteInvitation(identifier: string) {
      await api.delete(`/invitations/${identifier}`);
      this.invitations = this.invitations.filter(
        (i) => i.identifier !== identifier,
      );
      this.invitationCount = Math.max(0, this.invitationCount - 1);
      if (this.currentInvitation?.identifier === identifier)
        this.currentInvitation = null;
    },
  },
});

export { useInvitationsStore };
