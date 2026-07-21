<script setup lang="ts">
import { useUserInformationStore } from "~/stores/users";
import { useMarketplaceStore } from "~/stores/marketplace";
import { useTeamsStore } from "~/stores/teams";
import { useUploadStore } from "~/stores/uploads";
import api from "~/plugin/api";
import type { OrderStatus } from "~/bindings/OrderStatus";

useHead({ title: "Dashboard" });

definePageMeta({
  layout: "dashboard",
  hideBreadcrumb: true,
  breadcrumb: {
    icon: "heroicons:squares-2x2",
    ariaLabel: "dashboard",
    title: "Dashboard",
  },
});
const userStore = useUserInformationStore();
const marketplaceStore = useMarketplaceStore();
const teamsStore = useTeamsStore();
const uploadStore = useUploadStore();
const orderStore = useOrdersStore();

const totalProducts = ref(0);

onMounted(async () => {
  await Promise.all([
    marketplaceStore.fetchMarketplaces(),
    teamsStore.fetchAllMembers(),
    uploadStore.countUploads(),
    // orderStore.countByStatus("Pending" as OrderStatus),
    // orderStore.countByStatus("Pending" as OrderStatus),
  ]);

  let count = 0;
  for (const mp of marketplaceStore.marketplaces) {
    try {
      const { data } = await api.get(`/marketplaces/${mp.identifier}/products`);
      count += data?.products?.length ?? 0;
    } catch {
      /* skip failed */
    }
  }
  totalProducts.value = count;
});

const firstName = computed(() => userStore.userFirstName || "there");
const marketplaces = computed(() => marketplaceStore.marketplaces);
const totalMarketplaces = computed(() => marketplaces.value.length);

const stats = computed(() => [
  {
    label: "Marketplaces",
    value: totalMarketplaces.value,
    icon: "heroicons:building-storefront",
  },
  {
    label: "Total Products",
    value: totalProducts.value,
    icon: "heroicons:tag",
  },
  {
    label: "Team Members",
    value: teamsStore.members.length,
    icon: "heroicons:users",
  },
  {
    label: "Customers",
    value: teamsStore.members.length,
    icon: "heroicons:users",
  },
  {
    label: "Pending Orders",
    value: teamsStore.members.length,
    icon: "heroicons:users",
  },
  {
    label: "Uploads",
    value: uploadStore.count,
    icon: "heroicons:arrow-up-tray",
  },
]);

// Bar chart mock data — monthly activity (Jan–Jun)
const chartMonths = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const chartValues = [42, 68, 55, 80, 63, 91];
const chartMax = Math.max(...chartValues);

function barHeight(val: number) {
  return Math.round((val / chartMax) * 100);
}

// Quick links
const quickLinks = [
  {
    label: "Marketplace",
    path: "/marketplace",
    icon: "heroicons:building-storefront",
  },
  { label: "Uploads", path: "/uploads", icon: "heroicons:arrow-up-tray" },
  { label: "Team", path: "/teams", icon: "heroicons:users" },
  { label: "Metrics", path: "/metrics", icon: "heroicons:chart-bar-square" },
  { label: "Calendar", path: "/calendar", icon: "heroicons:calendar-days" },
  { label: "Settings", path: "/settings", icon: "heroicons:cog-6-tooth" },
];

// Recent marketplaces — latest 5
const recentMarketplaces = computed(() =>
  [...marketplaces.value]
    .sort((a, b) => {
      const da = a.createdAt ? new Date(a.createdAt).getTime() : 0;
      const db = b.createdAt ? new Date(b.createdAt).getTime() : 0;
      return db - da;
    })
    .slice(0, 5),
);
</script>

<template>
  <div class="space-y-8">
    <!-- Greeting row -->
    <div class="flex items-start justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
          Hi, {{ firstName }} 👋 <UIcon name="i-lucide:arrow-up-tray" />
        </h1>
        <p class="text-sm text-gray-400 dark:text-white/40 mt-1">
          Here's what's happening across your workspace today.
        </p>
      </div>
    </div>

    <!-- Stat cards + chart -->
    <div class="grid grid-cols-1 xl:grid-cols-3 gap-6">
      <!-- Stat cards (2/3 width) -->
      <div class="xl:col-span-2 grid grid-cols-2 gap-4">
        <AppStatCard
          v-for="stat in stats"
          :key="stat.label"
          :label="stat.label"
          :value="stat.value.toLocaleString()"
          :icon="stat.icon"
        />
      </div>

      <!-- Activity chart (1/3 width) -->
      <div class="border border-gray-100 dark:border-white/5 rounded-2xl p-5">
        <div class="flex items-center justify-between mb-4">
          <p class="text-sm font-semibold text-gray-700 dark:text-white/80">
            Activity
          </p>
          <span class="text-xs text-gray-400 dark:text-white/30"
            >Last 6 months</span
          >
        </div>

        <!-- Bar chart -->
        <div class="flex items-end gap-2 h-28">
          <template v-for="(val, i) in chartValues" :key="chartMonths[i]">
            <div class="flex-1 flex flex-col items-center gap-1">
              <div
                class="w-full rounded-md transition-all"
                :class="
                  i === chartValues.length - 1
                    ? 'bg-brand'
                    : 'bg-brand-100 dark:bg-brand/20'
                "
                :style="{ height: `${barHeight(val)}%` }"
              />
              <span
                class="text-[9px] text-gray-400 dark:text-white/25 font-medium"
              >
                {{ chartMonths[i] }}
              </span>
            </div>
          </template>
        </div>

        <div class="mt-4 pt-4 border-t border-gray-100 dark:border-white/5">
          <p class="text-lg font-bold text-gray-900 dark:text-white">
            {{ chartValues[chartValues.length - 1] }}
          </p>
          <p class="text-xs text-gray-400 dark:text-white/30">
            entries this month
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
