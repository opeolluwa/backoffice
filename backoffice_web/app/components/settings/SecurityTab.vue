<script setup lang="ts">
import type { FormSubmitEvent } from "@nuxt/ui";
import * as v from "valibot";
import { useUserInformationStore } from "~/stores/users";

const userStore = useUserInformationStore();
const toast = useToast();

const schema = v.pipe(
  v.object({
    currentPassword: v.pipe(
      v.string(),
      v.minLength(1, "Current password is required."),
    ),
    newPassword: v.pipe(
      v.string(),
      v.minLength(8, "Password must be at least 8 characters."),
    ),
    confirmPassword: v.pipe(
      v.string(),
      v.minLength(1, "Please confirm your password."),
    ),
  }),
  v.forward(
    v.partialCheck(
      [["newPassword"], ["confirmPassword"]],
      (input) => input.newPassword === input.confirmPassword,
      "Passwords do not match.",
    ),
    ["confirmPassword"],
  ),
);

type Schema = v.InferOutput<typeof schema>;

const state = reactive<Schema>({
  currentPassword: "",
  newPassword: "",
  confirmPassword: "",
});

const loading = ref(false);

async function onSubmit({ data: _ }: FormSubmitEvent<Schema>) {
  loading.value = true;
  try {
    await userStore.changePassword({
      currentPassword: state.currentPassword,
      newPassword: state.newPassword,
      confirmPassword: state.confirmPassword,
    });
    toast.add({ title: "Password changed", color: "success" });
    state.currentPassword = "";
    state.newPassword = "";
    state.confirmPassword = "";
  } catch {
    toast.add({ title: "Failed to change password", color: "error" });
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
      <p class="font-semibold text-gray-900 dark:text-white mb-1">
        Change password
      </p>
      <p class="text-xs text-gray-400 dark:text-white/30 mb-5">
        You'll be asked to sign in again after changing your password.
      </p>

      <UForm
        :schema="schema"
        :state="state"
        class="space-y-4"
        :on-submit="onSubmit"
      >
        <AppInput
          v-model="state.currentPassword"
          label="Current password"
          name="currentPassword"
          type="password"
          placeholder="••••••••"
        />

        <div
          class="border-t border-gray-100 dark:border-white/5 pt-4 space-y-4"
        >
          <AppInput
            v-model="state.newPassword"
            label="New password"
            name="newPassword"
            type="password"
            placeholder="••••••••"
          />
          <AppInput
            v-model="state.confirmPassword"
            label="Confirm new password"
            name="confirmPassword"
            type="password"
            placeholder="••••••••"
          />
        </div>

        <div class="pt-1">
          <AppButton type="submit" size="lg" :loading="loading" :disabled="loading">
            Change password
          </AppButton>
        </div>
      </UForm>
    </div>
  </div>
</template>
