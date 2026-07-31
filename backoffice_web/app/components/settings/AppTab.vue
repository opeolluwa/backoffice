<script setup lang="ts">
import type { FormSubmitEvent } from "@nuxt/ui";
import * as v from "valibot";
import { useAppStore } from "~/stores/app";
import { useCountryStore } from "~/stores/country";
import { useUploadStore } from "~/stores/uploads";
import { UPLOAD_LIMIT_SIZE } from "~/plugin/api";

const toast = useToast();
const appStore = useAppStore();
const countryStore = useCountryStore();
const uploadStore = useUploadStore();

const config = computed(() => appStore.config);

const schema = v.object({
  appName: v.pipe(v.string(), v.minLength(1, "App name is required.")),
  supportEmail: v.pipe(
    v.string(),
    v.email("Please enter a valid email address."),
  ),
});

type Schema = v.InferOutput<typeof schema>;

const state = reactive<Schema>({
  appName: config.value?.appName ?? "",
  supportEmail: config.value?.supportEmail ?? "",
});

const defaultCurrency = ref(config.value?.defaultCurrency ?? "");
const defaultLanguage = ref(config.value?.defaultLanguage ?? "en");

const languages = [
  { label: "English", value: "en" },
  { label: "Spanish", value: "es" },
  { label: "French", value: "fr" },
  { label: "German", value: "de" },
  { label: "Portuguese", value: "pt" },
];

const currencyOptions = computed(() =>
  countryStore.countries.map((c) => ({
    label: `${c.currencyCode} - ${c.country}`,
    avatar: c.flag ? c.flag : undefined,
    value: c.identifier,
  })),
);

function resolveCurrencyIdentifier(value: string): string {
  const needle = value.toLowerCase();
  return (
    countryStore.countries.find(
      (c) =>
        c.identifier.toLowerCase() === needle ||
        c.currencyCode.toLowerCase() === needle,
    )?.identifier ?? ""
  );
}

const loading = ref(false);
const localeLoading = ref(false);
const logoLoading = ref(false);
const logoFile = ref<File | null>(null);

function formatFullDate(date: string | null | undefined): string {
  if (!date) return "-";
  return new Date(date).toLocaleDateString("en-US", {
    weekday: "long",
    year: "numeric",
    month: "long",
    day: "numeric",
  });
}

onMounted(async () => {
  await Promise.all([appStore.fetchConfig(), countryStore.fetchCountries()]);
  state.appName = config.value?.appName ?? "";
  state.supportEmail = config.value?.supportEmail ?? "";
  defaultCurrency.value = resolveCurrencyIdentifier(
    config.value?.defaultCurrency ?? "",
  );
  defaultLanguage.value = config.value?.defaultLanguage ?? "en";
});

async function onSubmit({ data }: FormSubmitEvent<Schema>) {
  loading.value = true;
  try {
    await appStore.updateConfig({
      appName: data.appName,
      supportEmail: data.supportEmail,
    });
    state.appName = data.appName;
    state.supportEmail = data.supportEmail;
    toast.add({ title: "App settings updated", color: "success" });
  } catch {
    toast.add({ title: "Failed to update app settings", color: "error" });
  } finally {
    loading.value = false;
  }
}

async function saveLocale() {
  localeLoading.value = true;
  try {
    await appStore.updateConfig({
      defaultCurrency: defaultCurrency.value || null,
      defaultLanguage: defaultLanguage.value || null,
    });
    toast.add({ title: "Locale settings saved", color: "success" });
  } catch {
    toast.add({ title: "Failed to save locale settings", color: "error" });
  } finally {
    localeLoading.value = false;
  }
}

async function saveLogo() {
  if (!logoFile.value) {
    toast.add({ title: "Please select a logo image", color: "warning" });
    return;
  }
  if (logoFile.value.size > UPLOAD_LIMIT_SIZE) {
    toast.add({
      title: "Logo file is too large",
      description: "Maximum size is 25 MB.",
      color: "error",
    });
    return;
  }
  logoLoading.value = true;
  try {
    const created = await uploadStore.createUpload({
      file: logoFile.value,
      name: logoFile.value.name,
    });
    if (!created || typeof created !== "object" || !("url" in created)) {
      throw new Error("Upload failed");
    }
    await appStore.updateConfig({ logoUrl: created.url as string });
    logoFile.value = null;
    toast.add({ title: "Logo saved", color: "success" });
  } catch {
    toast.add({
      title: "Failed to save logo",
      description: "Please try again.",
      color: "error",
    });
  } finally {
    logoLoading.value = false;
  }
}

async function removeLogo() {
  logoLoading.value = true;
  try {
    await appStore.updateConfig({ logoUrl: null });
    logoFile.value = null;
    toast.add({ title: "Logo removed", color: "success" });
  } catch {
    toast.add({ title: "Failed to remove logo", color: "error" });
  } finally {
    logoLoading.value = false;
  }
}

async function toggleMaintenance() {
  if (!config.value) return;
  const newValue = !config.value.maintenanceMode;
  try {
    await appStore.updateConfig({ maintenanceMode: newValue });
    toast.add({
      title: newValue
        ? "Maintenance mode enabled"
        : "Maintenance mode disabled",
      color: newValue ? "warning" : "success",
    });
  } catch {
    toast.add({ title: "Failed to toggle maintenance mode", color: "error" });
  }
}
</script>

<template>
  <div class="space-y-4">
    <!-- General -->
    <div
      class="bg-white dark:bg-gray-800 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
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

    <!-- Logo -->
    <div
      class="bg-white dark:bg-gray-800 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
    >
      <p class="font-semibold text-gray-900 dark:text-white mb-1">Logo</p>
      <p class="text-xs text-gray-400 dark:text-white/30 mb-5">
        Upload a logo for your application. It will be used across the app.
      </p>

      <div class="space-y-4">
        <div class="flex items-start gap-4">
          <div
            class="w-20 h-20 rounded-xl border border-gray-200 dark:border-white/10 overflow-hidden flex items-center justify-center bg-gray-50 dark:bg-white/5 shrink-0"
          >
            <img
              v-if="config?.logoUrl"
              :src="config.logoUrl"
              :alt="config?.appName || 'App logo'"
              class="w-full h-full object-cover"
            />
            <UIcon
              v-else
              name="heroicons:photo"
              class="size-7 text-gray-300 dark:text-white/20"
            />
          </div>
          <div class="flex-1 min-w-0">
            <UFileUpload v-model="logoFile" accept="image/*" class="w-full" />
            <p class="text-xs text-gray-400 dark:text-white/30 mt-2">
              PNG, JPG or SVG up to 25 MB.
            </p>
          </div>
        </div>

        <div class="flex items-center gap-2 pt-1">
          <AppButton
            size="lg"
            :loading="logoLoading"
            :disabled="logoLoading || !logoFile"
            @click="saveLogo"
          >
            Save logo
          </AppButton>
          <AppButton
            v-if="config?.logoUrl"
            size="lg"
            color="error"
            :loading="logoLoading"
            :disabled="logoLoading"
            @click="removeLogo"
          >
            Remove logo
          </AppButton>
        </div>
      </div>
    </div>

    <!-- Locale Defaults -->
    <div
      class="bg-white dark:bg-gray-800 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
    >
      <p class="font-semibold text-gray-900 dark:text-white mb-1">
        Locale defaults
      </p>
      <p class="text-xs text-gray-400 dark:text-white/30 mb-5">
        Set the default currency and language for the application.
      </p>

      <div class="space-y-4">
        <AppSelect
          v-model="defaultCurrency"
          :items="currencyOptions"
          label="Default currency"
          placeholder="Select currency"
          class="w-full"
        />

        <AppSelect
          v-model="defaultLanguage"
          :items="languages"
          label="Default language"
          placeholder="Select language"
          class="w-full"
        />

        <div class="pt-1">
          <AppButton
            size="lg"
            :loading="localeLoading"
            :disabled="localeLoading"
            @click="saveLocale"
          >
            Save locale
          </AppButton>
        </div>
      </div>
    </div>

    <!-- Maintenance -->
    <div
      class="bg-white dark:bg-gray-800 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
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
              {{ config?.maintenanceMode ? "Currently active" : "Inactive" }}
            </p>
          </div>
        </div>
        <USwitch
          :model-value="config?.maintenanceMode ?? false"
          @update:model-value="toggleMaintenance"
        />
      </div>

      <div
        v-if="config?.maintenanceMode"
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
      class="bg-white dark:bg-gray-800 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
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
            config?.identifier
          }}</span>
        </div>
        <div class="border-t border-gray-100 dark:border-white/5" />
        <div class="flex items-center justify-between">
          <span class="text-sm text-gray-500 dark:text-white/40">Created</span>
          <span class="text-sm font-medium text-gray-700 dark:text-white/60">
            {{ formatFullDate(config?.createdAt) }}
          </span>
        </div>
        <div class="border-t border-gray-100 dark:border-white/5" />
        <div class="flex items-center justify-between">
          <span class="text-sm text-gray-500 dark:text-white/40"
            >Last updated</span
          >
          <span class="text-sm font-medium text-gray-700 dark:text-white/60">
            {{ formatFullDate(config?.lastUpdated) }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
