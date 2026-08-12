<script setup lang="ts">
import type { FormSubmitEvent } from "@nuxt/ui";
import * as v from "valibot";
import { useUserInformationStore } from "~/stores/users";

const userStore = useUserInformationStore();
const toast = useToast();

const schema = v.object({
  firstName: v.pipe(v.string(), v.minLength(1, "First name is required.")),
  lastName: v.pipe(v.string(), v.minLength(1, "Last name is required.")),
  username: v.pipe(
    v.string(),
    v.minLength(2, "Username must be at least 2 characters."),
  ),
  email: v.pipe(v.string(), v.email("Please enter a valid email address.")),
});

type Schema = v.InferOutput<typeof schema>;

const state = reactive<Schema>({
  firstName: userStore.user.firstName ?? "",
  lastName: userStore.user.lastName ?? "",
  username: userStore.user.username ?? "",
  email: userStore.user.email ?? "",
});

const loading = ref(false);
const open = ref(false);
const file = ref<File | null>(null);
const uploading = ref(false);

async function onUpload() {
  if (!file.value) return;
  uploading.value = true;
  try {
    await userStore.uploadProfilePicture(file.value);
    toast.add({ title: "Profile picture updated", color: "success" });
    open.value = false;
    file.value = null;
  } catch {
    toast.add({ title: "Failed to upload profile picture", color: "error" });
  } finally {
    uploading.value = false;
  }
}

async function onSubmit({ data }: FormSubmitEvent<Schema>) {
  loading.value = true;
  try {
    await userStore.updateProfile({
      firstName: data.firstName,
      lastName: data.lastName,
      username: data.username,
    });
    state.firstName = userStore.user.firstName;
    state.lastName = userStore.user.lastName;
    state.username = userStore.user.username;
    state.email = userStore.user.email;
    toast.add({ title: "Profile updated", color: "success" });
  } catch {
    toast.add({ title: "Failed to update profile", color: "error" });
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="space-y-4">
    <div
      class=" border border-gray-100 dark:border-white/5 rounded-2xl p-5"
    >
      <UChip size="sm" position="bottom-right" class="bg-none">
        <template #content>
          <UButton
            color="neutral"
            icon="heroicons:pencil"
            class="cursor-pointer"
            @click="open = true"
          />
        </template>
        <UAvatar
          class="rounded-4xl"
          :src="String(userStore.profilePicture)"
          :alt="userStore.fullName"
          loading="lazy"
          height="420px"
          width="420px"
          :ui="{ root: 'h-36 w-36 rounded-full' }"
        />
      </UChip>
    </div>

    <div
      class=" border border-gray-100 dark:border-white/5 rounded-2xl p-5"
    >
      <p class="font-semibold text-gray-900 dark:text-white mb-1">
        Personal information
      </p>
      <p class="text-xs text-gray-400 dark:text-white/30 mb-5">
        Update your name, username, and email.
      </p>

      <UForm
        :schema="schema"
        :state="state"
        class="space-y-4"
        :on-submit="onSubmit"
      >
        <div class="grid grid-cols-2 gap-4">
          <AppInput
            v-model="state.firstName"
            label="First name"
            name="firstName"
            placeholder="Jane"
          />
          <AppInput
            v-model="state.lastName"
            label="Last name"
            name="lastName"
            placeholder="Doe"
          />
        </div>

        <AppInput
          v-model="state.username"
          label="Username"
          name="username"
          placeholder="janedoe"
        />

        <AppInput
          v-model="state.email"
          label="Email address"
          name="email"
          placeholder="jane@example.com"
          :disabled="true"
        />

        <div class="pt-1">
          <AppButton
            type="submit"
            size="lg"
            :loading="loading"
            :disabled="loading"
          >
            Save changes
          </AppButton>
        </div>
      </UForm>
    </div>

    <UModal
      v-model:open="open"
      title="Update profile picture"
      description="Upload a new profile picture."
    >
      <template #body>
        <UFileUpload v-model="file" class="w-full min-h-48" />
      </template>
      <template #footer>
        <div class="flex justify-end gap-2">
          <UButton
            color="neutral"
            variant="outline"
            label="Cancel"
            @click="open = false"
          />
          <UButton
            label="Upload"
            :loading="uploading"
            :disabled="!file"
            @click="onUpload"
          />
        </div>
      </template>
    </UModal>
  </div>
</template>
