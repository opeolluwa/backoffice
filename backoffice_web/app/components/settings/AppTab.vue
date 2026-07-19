<script setup lang="ts">
import type { FormSubmitEvent } from "@nuxt/ui";
import * as v from "valibot";
import type { AppConfigInterface } from "@bindings/AppConfigInterface";

const toast = useToast();

const config = reactive<AppConfigInterface>({
  identifier: 1,
  appName: "Backoffice",
  maintenanceMode: false,
  supportEmail: "support@example.com",
  createdAt: new Date().toISOString(),
  lastUpdated: new Date().toISOString(),
});

const schema = v.object({
  appName: v.pipe(v.string(), v.minLength(1, "App name is required.")),
  supportEmail: v.pipe(
    v.string(),
    v.email("Please enter a valid email address."),
  ),
});

type Schema = v.InferOutput<typeof schema>;

const state = reactive<Schema>({
  appName: config.appName ?? "",
  supportEmail: config.supportEmail ?? "",
});

const loading = ref(false);

async function onSubmit({ data }: FormSubmitEvent<Schema>) {
  loading.value = true;
  try {
    config.appName = data.appName;
    config.supportEmail = data.supportEmail;
    config.lastUpdated = new Date().toISOString();
    toast.add({ title: "App settings updated", color: "success" });
  } catch {
    toast.add({ title: "Failed to update app settings", color: "error" });
  } finally {
    loading.value = false;
  }
}

function toggleMaintenance() {
  config.maintenanceMode = !config.maintenanceMode;
  toast.add({
    title: config.maintenanceMode
      ? "Maintenance mode enabled"
      : "Maintenance mode disabled",
    color: config.maintenanceMode ? "warning" : "success",
  });
}
</script>

<template>
  <div class="space-y-4">
    <!-- General -->
    <div
      class="bg-white dark:bg-brand-dark-600 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
    >
      <p class="font-semibold text-gray-900 dark:text-white mb-1">General</p>
      <p class="text-xs text-gray-400 dark:text-white/30 mb-5">
        Basic application settings.
      </p>

      <UForm
        :schema="schema"
        :state="state"
        class="space-y-4"
        :on-submit="onSubmit"
      >
        <AppInput
          v-model="state.appName"
          label="App name"
          name="appName"
          placeholder="My App"
        />

        <AppInput
          v-model="state.supportEmail"
          label="Support email"
          name="supportEmail"
          placeholder="support@example.com"
        />

        <div class="pt-1">
          <AppButton type="submit" :loading="loading" :disabled="loading">
            Save changes
          </AppButton>
        </div>
      </UForm>
    </div>

    <!-- Maintenance -->
    <div
      class="bg-white dark:bg-brand-dark-600 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
    >
      <p class="font-semibold text-gray-900 dark:text-white mb-1">
        Maintenance
      </p>
      <p class="text-xs text-gray-400 dark:text-white/30 mb-5">
        Temporarily disable access for non-admin users.
      </p>

      <div
        class="flex items-center justify-between px-4 py-3.5 rounded-xl border border-gray-100 dark:border-white/5"
      >
        <div class="flex items-center gap-3">
          <div
            class="w-8 h-8 rounded-lg bg-gray-50 dark:bg-white/5 flex items-center justify-center shrink-0"
          >
            <UIcon
              name="heroicons:wrench-screwdriver"
              class="size-4 text-gray-400 dark:text-white/25"
            />
          </div>
          <div>
            <p class="text-sm font-medium text-gray-800 dark:text-white/80">
              Maintenance mode
            </p>
            <p class="text-xs text-gray-400 dark:text-white/30 mt-0.5">
              {{ config.maintenanceMode ? "Currently active" : "Inactive" }}
            </p>
          </div>
        </div>
        <UToggle
          :model-value="config.maintenanceMode"
          @update:model-value="toggleMaintenance"
        />
      </div>

      <div
        v-if="config.maintenanceMode"
        class="mt-3 flex items-start gap-2 p-3 rounded-lg bg-amber-50 dark:bg-amber-500/10"
      >
        <UIcon
          name="heroicons:exclamation-triangle"
          class="size-4 text-amber-500 shrink-0 mt-0.5"
        />
        <p class="text-xs text-amber-600 dark:text-amber-400">
          Maintenance mode is active. Non-admin users cannot access the
          application.
        </p>
      </div>
    </div>

    <!-- Info -->
    <div
      class="bg-white dark:bg-brand-dark-600 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
    >
      <p class="font-semibold text-gray-900 dark:text-white mb-1">App info</p>
      <p class="text-xs text-gray-400 dark:text-white/30 mb-5">
        Read-only information about this instance.
      </p>

      <div class="space-y-3">
        <div class="flex items-center justify-between">
          <span class="text-sm text-gray-500 dark:text-white/40"
            >Identifier</span
          >
          <span class="text-sm font-medium text-gray-700 dark:text-white/60">{{
            config.identifier
          }}</span>
        </div>
        <div class="border-t border-gray-100 dark:border-white/5" />
        <div class="flex items-center justify-between">
          <span class="text-sm text-gray-500 dark:text-white/40">Created</span>
          <span class="text-sm font-medium text-gray-700 dark:text-white/60">
            {{ new Date(config.createdAt).toLocaleDateString() }}
          </span>
        </div>
        <div class="border-t border-gray-100 dark:border-white/5" />
        <div class="flex items-center justify-between">
          <span class="text-sm text-gray-500 dark:text-white/40"
            >Last updated</span
          >
          <span class="text-sm font-medium text-gray-700 dark:text-white/60">
            {{ new Date(config.lastUpdated).toLocaleDateString() }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
