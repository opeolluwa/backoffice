import { defineStore } from "pinia";
import api from "~/plugin/api";
import type { UploadsInterface } from "~/bindings/UploadsInterface";
import type { CreateUploadRequest } from "~/bindings/CreateUploadRequest";

const useTeamsStore = defineStore("teams", {
  state: () => ({
    uploads: [] as Array<UploadsInterface>,
    starred: [] as Array<UploadsInterface>,
    count: 0,
  }),

  actions: {
    async createUpload(payload: CreateUploadRequest) {
    
    },
    async findAllUploads() {
    },
    async countUploads() {},
    async findStarredUploads() {},

    async getOneUpload() {},
    async updateOneUpload() {},
    async deleteOneUpload() {},
  },
  persist: true,
});

export { useTeamsStore };
