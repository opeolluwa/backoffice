import { defineStore } from "pinia";
import api from "~/plugin/api";
import type { EmailsInterface as Email } from "@bindings/EmailsInterface";

const useEmailStore = defineStore("emails", {
  state: () => ({
    emails: [] as Array<Email>,
    currentEmail: null as Email | null,
    emailCount: 0,
    unreadCount: 0,
  }),

  actions: {
    async createEmail(payload: {
      subject: string;
      body: string;
      senderEmail: string;
      recipientEmail: string;
      tag?: string | null;
      hasAttachments?: boolean | null;
      data?: Record<string, unknown> | null;
    }) {
      const res = await api.post("/emails", payload);
      const created = res.data?.data as Email;
      this.emails.unshift(created);
      this.emailCount++;
      return created;
    },

    async fetchEmails() {
      const res = await api.get("/emails");
      this.emails = (res.data?.data as Array<Email>) || [];
    },

    async fetchEmailById(identifier: string) {
      const res = await api.get(`/emails/${identifier}`);
      this.currentEmail = (res.data?.data as Email) || null;
      return this.currentEmail;
    },

    async fetchStarredEmails() {
      const res = await api.get("/emails/starred");
      this.emails = (res.data?.data as Array<Email>) || [];
    },

    async fetchUnreadEmails() {
      const res = await api.get("/emails/unread");
      this.emails = (res.data?.data as Array<Email>) || [];
    },

    async fetchEmailsByTag(tag: string) {
      const res = await api.get(`/emails/tag/${tag}`);
      this.emails = (res.data?.data as Array<Email>) || [];
    },

    async markAsRead(identifier: string) {
      await api.put(`/emails/${identifier}`, { isRead: true });
      const email = this.emails.find((e) => e.identifier === identifier);
      if (email) email.isRead = true;
      if (this.currentEmail?.identifier === identifier)
        this.currentEmail.isRead = true;
    },

    async toggleStarred(identifier: string, isStarred: boolean) {
      await api.put(`/emails/${identifier}`, { isStarred });
      const email = this.emails.find((e) => e.identifier === identifier);
      if (email) email.isStarred = isStarred;
      if (this.currentEmail?.identifier === identifier)
        this.currentEmail.isStarred = isStarred;
    },

    async updateEmail(
      identifier: string,
      payload: { tag?: string | null; isRead?: boolean; isStarred?: boolean },
    ) {
      const res = await api.put(`/emails/${identifier}`, payload);
      const updated = res.data?.data as Email;
      const idx = this.emails.findIndex((e) => e.identifier === identifier);
      if (idx !== -1) this.emails[idx] = updated;
      if (this.currentEmail?.identifier === identifier)
        this.currentEmail = updated;
      return updated;
    },

    async deleteEmail(identifier: string) {
      await api.delete(`/emails/${identifier}`);
      this.emails = this.emails.filter((e) => e.identifier !== identifier);
      this.emailCount = Math.max(0, this.emailCount - 1);
      if (this.currentEmail?.identifier === identifier)
        this.currentEmail = null;
    },

    async countEmails() {
      const res = await api.get("/emails/count");
      this.emailCount = (res.data?.data as number) || 0;
      return this.emailCount;
    },

    async countUnreadEmails() {
      const res = await api.get("/emails/count/unread");
      this.unreadCount = (res.data?.data as number) || 0;
      return this.unreadCount;
    },
  },

  persist: true,
});

export { useEmailStore };
