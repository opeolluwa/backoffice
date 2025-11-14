<script setup lang="ts">
definePageMeta({
  layout: "dashboard",
});

const openForm = ref(false);
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
        <UForm class="space-y-4">
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
  </div>
</template>

<style scoped></style>
