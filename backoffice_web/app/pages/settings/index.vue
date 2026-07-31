<script setup lang="ts">
useHead({ title: "Settings" });

definePageMeta({
  layout: "dashboard",
  breadcrumb: {
    icon: "heroicons:cog-6-tooth",
    ariaLabel: "Settings",
    title: "Settings",
  },
});

const route = useRoute();
type TabKey = "profile" | "security" | "locale" | "app";
const activeTab = ref<TabKey>((route.query.tab as TabKey) || "profile");

const tabs: { key: TabKey; label: string; icon: string; desc: string }[] = [
  {
    key: "app",
    label: "App",
    icon: "heroicons:cog-6-tooth",
    desc: "Application settings",
  },
  {
    key: "security",
    label: "Security",
    icon: "heroicons:lock-closed",
    desc: "Password & access",
  },
  {
    key: "profile",
    label: "Profile",
    icon: "heroicons:user-circle",
    desc: "Your personal info",
  },

  {
    key: "locale",
    label: "Locale",
    icon: "heroicons:computer-desktop",
    desc: "Device preferences",
  },
];
</script>

<template>
  <div class="flex gap-6 items-start">
    <aside class="w-52 shrink-0">
      <div
        class="bg-white dark:bg-gray-800 border border-gray-100 dark:border-white/5 rounded-2xl p-2 sticky top-0"
      >
        <p
          class="text-[10px] font-semibold uppercase tracking-widest text-gray-400 dark:text-white/25 px-3 py-2"
        >
          Settings
        </p>
        <nav class="space-y-0.5">
          <button
            v-for="tab in tabs"
            :key="tab.key"
            class="flex items-center gap-3 w-full px-3 py-2.5 rounded-xl text-sm transition-all text-left cursor-pointer"
            :class="
              activeTab === tab.key
                ? 'bg-accent/10 dark:bg-accent/15 text-accent font-medium'
                : 'text-gray-500 dark:text-white/40 hover:bg-gray-50 dark:hover:bg-white/5 hover:text-gray-800 dark:hover:text-white'
            "
            @click="activeTab = tab.key"
          >
            <UIcon :name="tab.icon" class="size-4 shrink-0" />
            <div class="min-w-0">
              <p class="leading-tight">{{ tab.label }}</p>
              <p
                class="text-[10px] leading-tight mt-0.5 truncate"
                :class="
                  activeTab === tab.key
                    ? 'text-accent/60'
                    : 'text-gray-400 dark:text-white/25'
                "
              >
                {{ tab.desc }}
              </p>
            </div>
          </button>
        </nav>
      </div>
    </aside>

    <div class="flex-1 min-w-0 max-w-xl">
      <SettingsAppTab v-show="activeTab === 'app'" />
      <SettingsSecurityTab v-show="activeTab === 'security'" />
      <SettingsProfileTab v-show="activeTab === 'profile'" />
      <SettingsLocaleTab v-show="activeTab === 'locale'" />
    </div>
  </div>
</template>
