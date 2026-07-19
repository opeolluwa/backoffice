<script setup lang="ts">
import * as v from "valibot";
import useLogout from "~/composables/useLogout";

const items = useBreadcrumbItems();

const searchInputRef = ref<HTMLInputElement | null>(null);

const isMac =
  typeof navigator !== "undefined" &&
  /mac|iphone|ipad|ipod/i.test(navigator.userAgent);

function focusSearch(e: KeyboardEvent) {
  const isTrigger =
    e.key === "f" && (isMac ? e.metaKey : e.ctrlKey) && !e.shiftKey;
  if (!isTrigger) return;
  e.preventDefault();
  searchInputRef.value?.focus();
}

onMounted(() => window.addEventListener("keydown", focusSearch));
onUnmounted(() => window.removeEventListener("keydown", focusSearch));

const routes = [
  {
    label: "Dashboard",
    icon: "heroicons:squares-2x2",
    to: "/home",
  },
  {
    label: "Operations",
    type: "label" as const,
  },
  {
    label: "Marketplace",
    icon: "heroicons:building-storefront",
    to: "/marketplace",
  },
  {
    label: "Uploads",
    icon: "heroicons:arrow-up-tray",
    to: "/uploads",
  },
  {
    label: "Metrics",
    icon: "heroicons:chart-bar-square",
    to: "/metrics",
  },
  {
    label: "Workspace",
    type: "label" as const,
  },
  {
    label: "Team",
    icon: "heroicons:users",
    to: "/teams",
  },
  {
    label: "Settings",
    icon: "heroicons:cog-6-tooth",
    to: "/settings",
  },
];

const schema = v.object({
  query: v.pipe(v.string()),
});

const state = reactive({
  query: "",
});

const logout = async () => useLogout();
</script>

<template>
  <UDashboardSidebar
    collapsible="icon"
    class="bg-primary-500 text-white border-r-0"
    :ui="{
      header: 'px-4 py-5',
      content: 'px-2 py-2',
      footer: 'px-3 pb-4 pt-3',
    }"
  >
    <template #header>
      <AppLogo />
    </template>

    <UNavigationMenu
      orientation="vertical"
      :items="routes"
      highlight
      highlight-color="white"
      class="w-full"
      :ui="{
        item: 'text-white/60 hover:text-white data-[active]:text-white',
        link: 'data-[active]:bg-white/15',
      }"
    />

    <template #footer>
      <button
        class="flex w-full items-center gap-3 px-3 py-2.5 rounded-lg text-sm text-white/60 hover:text-red-300 transition-all cursor-pointer"
        @click="logout"
      >
        <UIcon
          name="heroicons:arrow-left-start-on-rectangle"
          class="size-4 shrink-0"
        />
        <span>Sign out</span>
      </button>
    </template>
  </UDashboardSidebar>

  <!-- Main area -->
  <div class="flex-1 flex flex-col min-w-0 overflow-hidden">
    <!-- Header -->
    <header
      class="flex items-center justify-between px-8 py-3.5 border-b border-gray-100 dark:border-white/5 shrink-0"
    >
      <UForm :schema="schema" :state="state" class="w-80">
        <UFormField name="query">
          <UInput
            :ref="
              (el: any) =>
                (searchInputRef = el?.$el?.querySelector('input') ?? null)
            "
            v-model="state.query"
            placeholder="Search..."
            icon="heroicons:magnifying-glass"
            variant="outline"
            class="w-full"
            @keydown.escape="
              state.query = '';
              ($event.target as HTMLInputElement).blur();
            "
          >
            <template #trailing>
              <kbd
                class="hidden sm:inline-flex items-center gap-0.5 px-1.5 py-0.5 rounded border border-gray-200 dark:border-white/10 text-[10px] font-medium text-gray-400 dark:text-white/30 select-none"
              >
                {{ isMac ? "⌘" : "Ctrl" }}F
              </kbd>
            </template>
          </UInput>
        </UFormField>
      </UForm>

      <div class="flex items-center gap-4">
        <UIcon name="heroicons:bell" class="_icon" />
        <UColorModeButton />
        <NuxtLink to="/account">
          <UserCard />
        </NuxtLink>
      </div>
    </header>

    <!-- Page content -->
    <main class="flex-1 overflow-y-auto">
      <div class="px-8 pt-5">
        <UBreadcrumb
          :hide-non-existing="true"
          :hide-root="true"
          :items="items"
        >
          <template #separator>
            <span class="mx-2 text-gray-300 dark:text-white/20">/</span>
          </template>
        </UBreadcrumb>
      </div>
      <div class="px-8 py-8">
        <slot />
      </div>
    </main>
  </div>
</template>

<style scoped></style>
