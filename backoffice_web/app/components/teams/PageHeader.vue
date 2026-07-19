<template>
  <div class="flex items-start justify-between mb-8">
    <div>
      <h2 class="text-lg font-semibold">Team members</h2>
      <p class="text-sm text-muted mt-1">
        Manage your team and invitations.
      </p>
    </div>
    <UButton class="px-4 py-2 shrink-0" @click="$emit('invite')">
      Add team member
    </UButton>
  </div>
</template>
