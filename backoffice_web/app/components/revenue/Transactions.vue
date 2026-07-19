<script setup lang="ts">
export interface Transaction {
  id: string;
  description: string;
  date: string;
  amount: string;
  status: "completed" | "pending" | "failed";
}

defineProps<{
  transactions: Transaction[];
}>();
</script>

<template>
  <div
    class="bg-white dark:bg-brand-dark-600 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
  >
    <div class="flex items-center justify-between mb-5">
      <div>
        <p class="text-sm font-semibold text-gray-700 dark:text-white/80">
          Recent transactions
        </p>
        <p class="text-xs text-gray-400 dark:text-white/30 mt-0.5">
          Latest incoming payments
        </p>
      </div>
    </div>

    <div
      v-if="transactions.length === 0"
      class="flex flex-col items-center justify-center py-12 gap-4 text-center"
    >
      <div
        class="w-14 h-14 rounded-2xl bg-gray-50 dark:bg-white/5 flex items-center justify-center"
      >
        <UIcon
          name="heroicons:banknotes"
          class="size-7 text-gray-300 dark:text-white/20"
        />
      </div>
      <div>
        <p class="font-medium text-sm text-gray-700 dark:text-white/50">
          No transactions yet
        </p>
        <p class="text-xs text-muted mt-1">
          Transactions will appear here once payments come in.
        </p>
      </div>
    </div>

    <div v-else class="divide-y divide-gray-50 dark:divide-white/5">
      <div
        v-for="tx in transactions"
        :key="tx.id"
        class="flex items-center gap-4 py-3"
      >
        <div
          class="w-9 h-9 rounded-xl bg-brand-50 dark:bg-brand/10 flex items-center justify-center shrink-0"
        >
          <UIcon name="heroicons:banknotes" class="size-4 text-brand" />
        </div>
        <div class="flex-1 min-w-0">
          <p
            class="text-sm font-medium text-gray-800 dark:text-white/90 truncate"
          >
            {{ tx.description }}
          </p>
          <p class="text-xs text-gray-400 dark:text-white/30">
            {{ tx.date }}
          </p>
        </div>
        <div class="flex items-center gap-3 shrink-0">
          <span
            class="text-sm font-semibold text-gray-800 dark:text-white/80"
          >{{ tx.amount }}</span>
          <span
            class="text-xs rounded-full px-2.5 py-0.5 font-medium"
            :class="{
              'bg-brand-50 dark:bg-brand/10 text-brand dark:text-brand-300':
                tx.status === 'completed',
              'bg-amber-50 dark:bg-amber-500/10 text-amber-600 dark:text-amber-400':
                tx.status === 'pending',
              'bg-red-50 dark:bg-red-500/10 text-red-500':
                tx.status === 'failed',
            }"
          >
            {{ tx.status }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
