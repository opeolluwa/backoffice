import { defineStore } from "pinia";
import api, { UPLOADS_TIMEOUT } from "~/plugin/api";
import type { UploadsInterface } from "~/bindings/UploadsInterface";
import type { CreateUploadRequest } from "~/bindings/CreateUploadRequest";
import { useRandomName } from "~/composables/useRandomName";

const useUploadStore = defineStore("uploads", {
  state: () => ({
    uploads: [] as Array<UploadsInterface>,
    starred: [] as Array<UploadsInterface>,
    currentUpload: null as UploadsInterface | null,
    count: 0,
  }),

  actions: {
    async createUpload(payload: Partial<CreateUploadRequest>) {
      const formData = new FormData();

      if (payload.file) {
        formData.append("file", payload.file);
      }

      formData.append("name", payload.name || useRandomName());
      if (payload.starred !== null && payload.starred !== undefined) {
        formData.append("starred", String(payload.starred));
      }

      try {
        const res = await api.post("/uploads", formData, {
          timeout: UPLOADS_TIMEOUT,
        });
        const created = res.data?.data as UploadsInterface;
        this.uploads.unshift(created);
        this.count++;

        return created;
      } catch (error) {
        return error;
      }
    },

    async findAllUploads() {
      const res = await api.get("/uploads");
      this.uploads = (res.data?.data as Array<UploadsInterface>) || [];
    },

    async countUploads() {
      const res = await api.get("/uploads/count");
      this.count = (res.data?.data as number) || 0;
      return this.count;
    },

    async findStarredUploads() {
      const res = await api.get("/uploads/starred");
      this.starred = (res.data?.data as Array<UploadsInterface>) || [];
    },

    async getOneUpload(identifier: string) {
      const res = await api.get(`/uploads/${identifier}`);
      this.currentUpload = (res.data?.data as UploadsInterface) || null;
      return this.currentUpload;
    },

    async updateOneUpload(
      identifier: string,
      payload: { name?: string; starred?: boolean | null },
    ) {
      const res = await api.put(`/uploads/${identifier}`, payload);
      const updated = res.data?.data as UploadsInterface;
      const idx = this.uploads.findIndex((u) => u.identifier === identifier);
      if (idx !== -1) this.uploads[idx] = updated;
      const sIdx = this.starred.findIndex((u) => u.identifier === identifier);
      if (sIdx !== -1) this.starred[sIdx] = updated;
      if (this.currentUpload?.identifier === identifier)
        this.currentUpload = updated;
      return updated;
    },

    async deleteOneUpload(identifier: string) {
      await api.delete(`/uploads/${identifier}`);
      this.uploads = this.uploads.filter((u) => u.identifier !== identifier);
      this.starred = this.starred.filter((u) => u.identifier !== identifier);
      this.count = Math.max(0, this.count - 1);
      if (this.currentUpload?.identifier === identifier)
        this.currentUpload = null;
    },
  },
  persist: true,
});

export { useUploadStore };
