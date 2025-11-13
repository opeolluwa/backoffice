<script setup lang="ts">
definePageMeta({
  layout: "dashboard",
});

const openForm = ref(false);
</script>

<template>
  <div class="flex flex-col justify-center items-center min-h-screen">
    <h1>You currently don&apos;t have any product</h1>
    <UButton
      @click="openForm = true"
      color="neutral"
      variant="outline"
      class="px-5 py-4 rounded mt-3 cursor-pointer flex items-center justify-center gap-x-2"
    >
      <UIcon name="heroicons:plus-circle" class="size-5" />
      Add product
    </UButton>
  </div>

  <UModal
    title="Create store"
    description="A store lets you manage your goods"
    close-icon="heroicons:x-mark"
    v-model:open="openForm"
  >
    <template #body>
      <UForm class="space-y-4">
        <UFormField
          label="Name"
          name="name"
          required
          v-slot="{ error }"
          :ui="{
            error: 'text-red-500 text-sm mt-1',
          }"
        >
          <UInput
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
          label="Description"
          name="email"
          required
          v-slot="{ error }"
          :ui="{
            error: 'text-red-500 text-sm mt-1',
          }"
        >
          <UInput
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
          label="Slug"
          name="email"
          hint="eg: cars, roses"
          required
          v-slot="{ error }"
          :ui="{
            error: 'text-red-500 text-sm mt-1',
          }"
        >
          <UInput
            :ui="{ base: 'py-4 px-6' }"
            :class="[
              'w-full transition-colors',
              error
                ? 'border-red-500 focus:border-red-500'
                : 'border-gray-300 focus:border-black',
            ]"
          />
        </UFormField>
        <UButton type="submit"> Continue </UButton>
      </UForm>
    </template>
  </UModal>
</template>

<style scoped></style>
