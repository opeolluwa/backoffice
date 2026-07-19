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
  <UDashboardGroup>
    <UDashboardSidebar
      collapsible
      :toggle="{ color: 'white', variant: 'ghost' }"
      class="bg-primary-500 border-r-0"
      :ui="{
        root: 'bg-primary-500 border-r-0',
        header: 'h-16',
        body: 'gap-1',
        footer: 'border-t border-white/10',
      }"
    >
      <template #header="{ collapsed }">
        <div class="flex items-center gap-2.5 overflow-hidden">
          <div
            class="size-8 rounded-lg bg-white/20 flex items-center justify-center shrink-0"
          >
            <UIcon
              name="heroicons:building-office-2"
              class="size-4.5 text-white"
            />
          </div>
          <Transition name="fade" mode="out-in">
            <span
              v-if="!collapsed"
              class="text-sm font-semibold text-white whitespace-nowrap"
            >
              backoffice
            </span>
          </Transition>
        </div>
      </template>

      <template #default="{ collapsed }">
        <UNavigationMenu
          orientation="vertical"
          :items="routes"
          highlight
          highlight-color="neutral"
          :collapsed="collapsed"
          class="w-full text-white"
          :ui="{
            item: 'text-white/50 hover:text-white data-active:text-white',
            link: 'text-white/50 hover:text-white data-active:text-white data-active:bg-white/15 rounded-lg',
            label: 'text-sm text-white/50 group-hover:text-white',
          }"
        />
      </template>

      <template #footer="{ collapsed }">
        <button
          class="flex w-full items-center gap-3 rounded-lg text-sm transition-all cursor-pointer"
          :class="
            collapsed
              ? 'justify-center text-white/60 hover:text-red-300 py-2'
              : 'px-3 py-2.5 text-white/60 hover:text-red-300'
          "
          @click="logout"
        >
          <UIcon
            name="heroicons:arrow-left-start-on-rectangle"
            class="size-4 shrink-0"
          />
          <Transition name="fade" mode="out-in">
            <span v-if="!collapsed">Sign out</span>
          </Transition>
        </button>
      </template>
    </UDashboardSidebar>

    <!-- Main area -->
    <div class="flex-1 flex flex-col min-w-0 overflow-hidden">
      <!-- Header -->
      <header
        class="flex items-center justify-between px-8 py-3.5 border-b border-gray-100 dark:border-white/5 shrink-0 h-16"
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
  </UDashboardGroup>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
