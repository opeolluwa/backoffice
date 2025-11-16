<script setup lang="ts">
import * as v from "valibot";
import type { FormSubmitEvent } from "@nuxt/ui";
import api from "~/plugin/api";
definePageMeta({
  layout: "dashboard",
});

const schema = v.object({
  name: v.pipe(v.string(), v.minLength(1, "Name is required ")),
  description: v.pipe(v.string(), v.minLength(1, "Description is required ")),
  slug: v.pipe(v.string(), v.minLength(1, "Slug is required ")),
});

type Schema = v.InferOutput<typeof schema>;

const openForm = ref(false);

const state = reactive<Schema>({
  name: "",
  description: "",
  slug: "",
});

const resetForm = () => {
  state.name = "";
  state.description = "";
  state.slug = "";
};

const toast = useToast();
const loading = ref(false);
async function onSubmit({ data }: FormSubmitEvent<Schema>) {
  loading.value = true;
  try {
    const { status, data: response } = api.post("/marketplace", data);
    if (status !== 201) {
      throw new Error(response?.message || "Failed to create marketplace");
    }

    // eslint-disable-next-line @typescript-eslint/no-unused-vars
  } catch (error) {
    toast.add({
      title: "Error",
      description: "Failed to create marketplace. Please try again.",
      color: "error",
    });
  } finally {
    loading.value = false;
  }

  toast.add({
    title: "Success",
    description: "Product added successfully.",
  });
  openForm.value = false;
  resetForm();
}
</script>

<template>
  <div>
    <div class="flex flex-col justify-center items-center h-[70vh]">
      <h1>You currently don&apos;t have any product</h1>
      <UButton
        color="neutral"
        variant="outline"
        class="px-5 py-4 rounded mt-3 cursor-pointer flex items-center justify-center gap-x-2"
        @click="openForm = true"
      >
        <UIcon name="heroicons:plus-circle" class="size-5" />
        Add product
      </UButton>
    </div>

    <UModal
      v-model:open="openForm"
      title="Create store"
      description="A store lets you manage your goods"
      close-icon="heroicons:x-mark"
    >
      <template #body>
        <UForm
          class="space-y-4"
          :schema="schema"
          :state="state"
          @submit.prevent="onSubmit"
        >
          <UFormField
            v-slot="{ error }"
            label="Name"
            name="name"
            required
            :ui="{
              error: 'text-red-500 text-sm mt-1',
            }"
          >
            <UInput
              v-model="state.name"
              :ui="{ base: 'py-4 px-6' }"
              :class="[
                'w-full transition-colors',
                error
                  ? 'border-red-500 focus:border-red-500'
                  : 'border-gray-300 focus:border-black',
              ]"
            />
          </UFormField>

          <UFormField
            v-slot="{ error }"
            label="Description"
            name="email"
            required
            :ui="{
              error: 'text-red-500 text-sm mt-1',
            }"
          >
            <UInput
              v-model="state.description"
              :ui="{ base: 'py-4 px-6' }"
              :class="[
                'w-full transition-colors',
                error
                  ? 'border-red-500 focus:border-red-500'
                  : 'border-gray-300 focus:border-black',
              ]"
            />
          </UFormField>

          <UFormField
            v-slot="{ error }"
            label="Slug"
            name="email"
            hint="eg: cars, roses"
            required
            :ui="{
              error: 'text-red-500 text-sm mt-1',
            }"
          >
            <UInput
              v-model="state.slug"
              :ui="{ base: 'py-4 px-6' }"
              :class="[
                'w-full transition-colors',
                error
                  ? 'border-red-500 focus:border-red-500'
                  : 'border-gray-300 focus:border-black',
              ]"
            />
          </UFormField>
          <div class="flex justify-between items-center">
            <UButton
              type="submit"
              class=""
              :loading="loading"
              :disabled="loading"
            >
              Continue
            </UButton>
            <UButton variant="subtle" color="error" class="" @click="resetForm">
              Clear form
            </UButton>
          </div>
        </UForm>
      </template>
    </UModal>
  </div>
</template>

<style scoped></style>
