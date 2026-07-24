<script setup lang="ts">
const toast = useToast();
const colorMode = useColorMode();

const themes = [
  { label: "Light", value: "light", icon: "heroicons:sun" },
  { label: "Dark", value: "dark", icon: "heroicons:moon" },
  { label: "System", value: "system", icon: "heroicons:computer-desktop" },
];

const selectedTheme = ref(
  themes.find((t) => t.value === colorMode.preference)?.value ?? "system",
);

function setTheme(value: string) {
  selectedTheme.value = value;
  colorMode.preference = value;
}

const languages = [
  { label: "English", value: "en" },
  { label: "Spanish", value: "es" },
  { label: "French", value: "fr" },
  { label: "German", value: "de" },
  { label: "Portuguese", value: "pt" },
];

const selectedLanguage = ref("en");

const timezones = [
  "UTC",
  "America/New_York",
  "America/Chicago",
  "America/Denver",
  "America/Los_Angeles",
  "Europe/London",
  "Europe/Paris",
  "Europe/Berlin",
  "Asia/Tokyo",
  "Asia/Shanghai",
  "Australia/Sydney",
];

const selectedTimezone = ref("UTC");

function save() {
  toast.add({ title: "Local settings saved", color: "success" });
}
</script>

<template>
  <div class="space-y-4">
    <!-- Theme -->
    <div
      class="hidden bg-white dark:bg-brand-dark-600 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
    >
      <p class="font-semibold text-gray-900 dark:text-white mb-1">Appearance</p>
      <p class="text-xs text-gray-400 dark:text-white/30 mb-5">
        Choose how the app looks on this device.
      </p>

      <div class="grid grid-cols-3 gap-3">
        <button
          v-for="theme in themes"
          :key="theme.value"
          class="flex flex-col items-center gap-2 p-4 rounded-xl border transition-all cursor-pointer"
          :class="
            selectedTheme === theme.value
              ? 'border-brand bg-brand-50 dark:bg-brand/10 text-brand'
              : 'border-gray-100 dark:border-white/5 hover:border-gray-200 dark:hover:border-white/10 text-gray-500 dark:text-white/40'
          "
          @click="setTheme(theme.value)"
        >
          <UIcon :name="theme.icon" class="size-5" />
          <span class="text-xs font-medium">{{ theme.label }}</span>
        </button>
      </div>
    </div>

    <!-- Language -->
    <div
      class="bg-white dark:bg-brand-dark-600 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
    >
      <p class="font-semibold text-gray-900 dark:text-white mb-1">Language</p>
      <p class="text-xs text-gray-400 dark:text-white/30 mb-5">
        Select your preferred language.
      </p>

      <AppSelect
        v-model="selectedLanguage"
        :items="languages"
        placeholder="Select language"
        class="w-full"
      />
    </div>

    <!-- Timezone -->
    <div
      class="bg-white dark:bg-brand-dark-600 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
    >
      <p class="font-semibold text-gray-900 dark:text-white mb-1">Timezone</p>
      <p class="text-xs text-gray-400 dark:text-white/30 mb-5">
        Used for scheduling and timestamps.
      </p>

      <AppSelect
        v-model="selectedTimezone"
        :items="timezones"
        placeholder="Select timezone"
        class="w-full"
      />
    </div>

    <div>
      <AppButton size="lg" @click="save">Save preferences</AppButton>
    </div>
  </div>
</template>
