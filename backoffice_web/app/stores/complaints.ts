import { defineStore } from "pinia";
import api from "~/plugin/api";
import type { ComplaintsInterface as Complaint } from "~/bindings/ComplaintsInterface";
import type { CustomersInterface as Customer } from "~/bindings/CustomersInterface";

interface CreateComplaintPayload {
  customerIdentifier: string;
  orderIdentifier?: string;
  subject: string;
  description: string;
}

interface UpdateComplaintPayload {
  subject?: string;
  description?: string;
  status?: string;
}

type ComplaintWithCustomer = [Complaint, Customer | null];

const useComplaintsStore = defineStore("complaints", {
  state: () => ({
    complaints: [] as Array<ComplaintWithCustomer>,
    currentComplaint: null as ComplaintWithCustomer | null,
    count: 0,
  }),

  getters: {
    complaintsList(): Array<Complaint> {
      return this.complaints.map(([c]) => c);
    },
  },

  actions: {
    async fetchComplaints() {
      const res = await api.get("/complaints");
      this.complaints = (res.data?.data as Array<ComplaintWithCustomer>) || [];
      this.count = this.complaints.length;
    },

    async createComplaint(payload: CreateComplaintPayload) {
      const res = await api.post("/complaints", payload);
      const created = res.data?.data as Complaint;
      this.complaints.unshift([created, null]);
      this.count++;
      return created;
    },

    async findComplaint(identifier: string) {
      const res = await api.get(`/complaints/${identifier}`);
      this.currentComplaint =
        (res.data?.data as ComplaintWithCustomer) || null;
      return this.currentComplaint;
    },

    async updateComplaint(
      identifier: string,
      payload: UpdateComplaintPayload,
    ) {
      const res = await api.put(`/complaints/${identifier}`, payload);
      const updated = res.data?.data as Complaint;
      const idx = this.complaints.findIndex(
        ([c]) => c.identifier === identifier,
      );
      if (idx !== -1) {
        this.complaints[idx] = [updated, this.complaints[idx][1]];
      }
      if (this.currentComplaint?.[0].identifier === identifier) {
        this.currentComplaint = [updated, this.currentComplaint[1]];
      }
      return updated;
    },

    async deleteComplaint(identifier: string) {
      await api.delete(`/complaints/${identifier}`);
      this.complaints = this.complaints.filter(
        ([c]) => c.identifier !== identifier,
      );
      this.count = Math.max(0, this.count - 1);
      if (this.currentComplaint?.[0].identifier === identifier)
        this.currentComplaint = null;
    },

    async countComplaints() {
      const res = await api.get("/complaints/count");
      this.count = res.data?.data ?? 0;
    },
  },

  persist: true,
});

export { useComplaintsStore };
export type { CreateComplaintPayload, UpdateComplaintPayload };
