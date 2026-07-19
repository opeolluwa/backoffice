<script setup lang="ts">
useHead({ title: "Revenue" });

definePageMeta({
  layout: "dashboard",
  breadcrumb: {
    icon: "heroicons:banknotes",
    ariaLabel: "Revenue",
    title: "Revenue",
  },
});

const stats = [
  {
    label: "Total Revenue",
    value: "$0",
    trend: "+0%",
    up: true,
    icon: "heroicons:banknotes",
    desc: "All time",
  },
  {
    label: "Monthly Recurring",
    value: "$0",
    trend: "+0%",
    up: true,
    icon: "heroicons:arrow-path",
    desc: "MRR",
  },
  {
    label: "Annual Recurring",
    value: "$0",
    trend: "+0%",
    up: true,
    icon: "heroicons:calendar-days",
    desc: "ARR",
  },
  {
    label: "Avg. Order Value",
    value: "$0",
    trend: "+0%",
    up: true,
    icon: "heroicons:receipt-percent",
    desc: "Per transaction",
  },
];

const chartMonths = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const chartValues = [0, 0, 0, 0, 0, 0];

const sources = [
  { label: "Marketplace sales", pct: 0, color: "bg-brand" },
  { label: "Subscriptions", pct: 0, color: "bg-brand-300" },
  { label: "One-time purchases", pct: 0, color: "bg-brand-100 dark:bg-brand/30" },
  { label: "Other", pct: 0, color: "bg-gray-200 dark:bg-white/10" },
];

const transactions: { id: string; description: string; date: string; amount: string; status: "completed" | "pending" | "failed" }[] = [];

const timeRanges = ["7 days", "30 days", "90 days", "12 months"] as const;
type TimeRange = (typeof timeRanges)[number];
const selectedRange = ref<TimeRange>("30 days");
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-start justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
          Revenue
        </h1>
        <p class="text-sm text-gray-400 dark:text-white/40 mt-1">
          Track income, transactions, and revenue streams.
        </p>
      </div>

      <div
        class="flex items-center gap-1 p-1 rounded-xl bg-gray-100 dark:bg-white/5"
      >
        <button
          v-for="range in timeRanges"
          :key="range"
          class="px-3 py-1.5 text-xs font-medium rounded-lg transition-all"
          :class="
            selectedRange === range
              ? 'bg-white dark:bg-white/10 text-gray-900 dark:text-white shadow-sm'
              : 'text-gray-500 dark:text-white/40 hover:text-gray-700 dark:hover:text-white/60'
          "
          @click="selectedRange = range"
        >
          {{ range }}
        </button>
      </div>
    </div>

    <div class="grid grid-cols-2 xl:grid-cols-4 gap-4">
      <RevenueStatCard v-for="stat in stats" :key="stat.label" v-bind="stat" />
    </div>

    <div class="grid grid-cols-1 xl:grid-cols-3 gap-6">
      <RevenueChart
        :months="chartMonths"
        :values="chartValues"
        :range="selectedRange"
      />
      <RevenueSources :sources="sources" />
    </div>

    <RevenueTransactions :transactions="transactions" />
  </div>
</template>
