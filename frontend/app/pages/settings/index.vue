<script setup lang="ts">
import type { FormSubmitEvent } from "@nuxt/ui";
import * as v from "valibot";
import { useUserInformationStore } from "~/stores/user";

definePageMeta({
  layout: "dashboard",
  breadcrumb: {
    icon: "heroicons:cog-6-tooth",
    ariaLabel: "Settings",
    title: "Settings",
  },
});

const userStore = useUserInformationStore();
const toast = useToast();

// ─── Tabs ─────────────────────────────────────────────────────────────────────

type TabKey = "profile" | "security" | "notifications";
const activeTab = ref<TabKey>("profile");

const tabs: { key: TabKey; label: string; icon: string }[] = [
  { key: "profile",       label: "Profile",       icon: "heroicons:user" },
  { key: "security",      label: "Security",       icon: "heroicons:lock-closed" },
  { key: "notifications", label: "Notifications",  icon: "heroicons:bell" },
];

// ─── Profile ──────────────────────────────────────────────────────────────────

const profileSchema = v.object({
  firstName: v.pipe(v.string(), v.minLength(1, "First name is required.")),
  lastName:  v.pipe(v.string(), v.minLength(1, "Last name is required.")),
  username:  v.pipe(v.string(), v.minLength(2, "Username must be at least 2 characters.")),
  email:     v.pipe(v.string(), v.email("Please enter a valid email address.")),
});

type ProfileSchema = v.InferOutput<typeof profileSchema>;

const profileState = reactive<ProfileSchema>({
  firstName: userStore.user.firstName ?? "",
  lastName:  userStore.user.lastName  ?? "",
  username:  userStore.user.username  ?? "",
  email:     userStore.user.email     ?? "",
});

const profileLoading = ref(false);

async function onProfileSubmit({ data }: FormSubmitEvent<ProfileSchema>) {
  profileLoading.value = true;
  try {
    // TODO: replace with actual API call
    userStore.updateProfile({ ...userStore.user, ...data });
    toast.add({ title: "Profile updated", color: "success" });
  } catch {
    toast.add({ title: "Failed to update profile", color: "error" });
  } finally {
    profileLoading.value = false;
  }
}

// ─── Security (password) ──────────────────────────────────────────────────────

const passwordSchema = v.pipe(
  v.object({
    currentPassword: v.pipe(v.string(), v.minLength(1, "Current password is required.")),
    newPassword:     v.pipe(v.string(), v.minLength(8, "Password must be at least 8 characters.")),
    confirmPassword: v.pipe(v.string(), v.minLength(1, "Please confirm your password.")),
  }),
  v.forward(
    v.partialCheck(
      [["newPassword"], ["confirmPassword"]],
      (input) => input.newPassword === input.confirmPassword,
      "Passwords do not match.",
    ),
    ["confirmPassword"],
  ),
);

type PasswordSchema = v.InferOutput<typeof passwordSchema>;

const passwordState = reactive<PasswordSchema>({
  currentPassword: "",
  newPassword:     "",
  confirmPassword: "",
});

const passwordLoading = ref(false);

async function onPasswordSubmit({ data: _ }: FormSubmitEvent<PasswordSchema>) {
  passwordLoading.value = true;
  try {
    // TODO: replace with actual API call
    toast.add({ title: "Password changed", color: "success" });
    passwordState.currentPassword = "";
    passwordState.newPassword     = "";
    passwordState.confirmPassword = "";
  } catch {
    toast.add({ title: "Failed to change password", color: "error" });
  } finally {
    passwordLoading.value = false;
  }
}

// ─── Notifications ────────────────────────────────────────────────────────────

const notifications = reactive({
  productUpdates:    true,
  teamActivity:      true,
  marketplaceAlerts: false,
  weeklyDigest:      true,
  securityAlerts:    true,
});

function saveNotifications() {
  // TODO: persist to API
  toast.add({ title: "Notification preferences saved", color: "success" });
}
</script>

<template>
  <div class="flex h-full min-h-0">

    <!-- ── Main content ──────────────────────────────────────────────────── -->
    <div class="flex-1 overflow-y-auto p-8">
      <h1 class="text-xl font-semibold mb-6">Settings</h1>

      <!-- Profile -->
      <template v-if="activeTab === 'profile'">
        <div class="rounded-xl border border-default bg-elevated p-6 max-w-lg">
          <p class="font-semibold mb-5">Profile</p>

          <!-- User identity row -->
          <div class="flex items-center gap-3 mb-6">
            <div class="w-10 h-10 rounded-full bg-gray-200 dark:bg-white/10 flex items-center justify-center shrink-0">
              <UIcon name="heroicons:user" class="size-5 text-muted" />
            </div>
            <span class="text-sm font-medium">
              {{ [profileState.firstName, profileState.lastName].filter(Boolean).join(" ") || "—" }}
            </span>
          </div>

          <UForm
            :schema="profileSchema"
            :state="profileState"
            class="space-y-4"
            @submit="onProfileSubmit"
          >
            <div class="grid grid-cols-2 gap-4">
              <UFormField label="First name" name="firstName" required :ui="{ error: 'text-red-500 text-sm mt-1' }">
                <UInput v-model="profileState.firstName" placeholder="Jane" class="w-full" />
              </UFormField>
              <UFormField label="Last name" name="lastName" required :ui="{ error: 'text-red-500 text-sm mt-1' }">
                <UInput v-model="profileState.lastName" placeholder="Doe" class="w-full" />
              </UFormField>
            </div>

            <UFormField label="Username" name="username" required :ui="{ error: 'text-red-500 text-sm mt-1' }">
              <UInput v-model="profileState.username" placeholder="janedoe" class="w-full">
                <template #leading>
                  <span class="text-muted text-sm">@</span>
                </template>
              </UInput>
            </UFormField>

            <UFormField label="Email address" name="email" required :ui="{ error: 'text-red-500 text-sm mt-1' }">
              <UInput v-model="profileState.email" placeholder="jane@example.com" class="w-full" />
            </UFormField>

            <div class="pt-1">
              <UButton type="submit" :loading="profileLoading" :disabled="profileLoading">
                Save changes
              </UButton>
            </div>
          </UForm>
        </div>
      </template>

      <!-- Security -->
      <template v-else-if="activeTab === 'security'">
        <div class="rounded-xl border border-default bg-elevated p-6 max-w-lg">
          <p class="font-semibold mb-1">Security</p>
          <p class="text-xs text-muted mb-5">Use a strong, unique password. Must be at least 8 characters.</p>

          <UForm
            :schema="passwordSchema"
            :state="passwordState"
            class="space-y-4"
            @submit="onPasswordSubmit"
          >
            <UFormField label="Current password" name="currentPassword" required :ui="{ error: 'text-red-500 text-sm mt-1' }">
              <UInput v-model="passwordState.currentPassword" type="password" placeholder="••••••••" class="w-full" />
            </UFormField>

            <UFormField label="New password" name="newPassword" required :ui="{ error: 'text-red-500 text-sm mt-1' }">
              <UInput v-model="passwordState.newPassword" type="password" placeholder="••••••••" class="w-full" />
            </UFormField>

            <UFormField label="Confirm new password" name="confirmPassword" required :ui="{ error: 'text-red-500 text-sm mt-1' }">
              <UInput v-model="passwordState.confirmPassword" type="password" placeholder="••••••••" class="w-full" />
            </UFormField>

            <div class="pt-1">
              <UButton type="submit" :loading="passwordLoading" :disabled="passwordLoading">
                Change password
              </UButton>
            </div>
          </UForm>
        </div>
      </template>

      <!-- Notifications -->
      <template v-else-if="activeTab === 'notifications'">
        <div class="rounded-xl border border-default bg-elevated p-6 max-w-lg">
          <p class="font-semibold mb-1">Notifications</p>
          <p class="text-xs text-muted mb-5">Choose which emails you'd like to receive.</p>

          <div class="space-y-3">
            <div
              v-for="item in [
                { key: 'productUpdates',    label: 'Product updates',    desc: 'News and feature announcements.' },
                { key: 'teamActivity',      label: 'Team activity',      desc: 'When members join, leave, or change roles.' },
                { key: 'marketplaceAlerts', label: 'Marketplace alerts', desc: 'Stock, pricing, and listing changes.' },
                { key: 'weeklyDigest',      label: 'Weekly digest',      desc: 'A summary of workspace activity.' },
                { key: 'securityAlerts',    label: 'Security alerts',    desc: 'Sign-ins from new devices or locations.' },
              ]"
              :key="item.key"
              class="flex items-center justify-between gap-4 p-3 rounded-lg bg-gray-50 dark:bg-white/[0.03] border border-default"
            >
              <div>
                <p class="text-sm font-medium">{{ item.label }}</p>
                <p class="text-xs text-muted mt-0.5">{{ item.desc }}</p>
              </div>
              <UToggle v-model="(notifications as any)[item.key]" />
            </div>

            <div class="pt-1">
              <UButton variant="outline" color="neutral" @click="saveNotifications">
                Save preferences
              </UButton>
            </div>
          </div>
        </div>
      </template>
    </div>

    <!-- ── Right panel ───────────────────────────────────────────────────── -->
    <div class="w-52 shrink-0 border-l border-default flex flex-col p-4">
      <p class="text-sm font-semibold mb-4">Panel</p>

      <p class="text-[11px] uppercase tracking-wider text-muted font-medium mb-2 px-1">
        Preferences
      </p>

      <nav class="space-y-0.5">
        <button
          v-for="tab in tabs"
          :key="tab.key"
          class="flex items-center gap-2.5 w-full px-3 py-2 rounded-lg text-sm font-medium transition-colors"
          :class="activeTab === tab.key
            ? 'bg-[var(--ui-primary)] text-white'
            : 'text-muted hover:bg-[var(--ui-bg-elevated)]'"
          @click="activeTab = tab.key"
        >
          <UIcon :name="tab.icon" class="size-4 shrink-0" />
          {{ tab.label }}
        </button>
      </nav>
    </div>

  </div>
</template>

<style scoped></style>
