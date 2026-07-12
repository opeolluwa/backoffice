<script setup lang="ts">
import type { FormSubmitEvent } from "@nuxt/ui";
import * as v from "valibot";
import { useUserInformationStore } from "~/stores/users";

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


type TabKey = "profile" | "security" | "notifications";
const activeTab = ref<TabKey>("profile");

const tabs: { key: TabKey; label: string; icon: string; desc: string }[] = [
  {
    key: "profile",
    label: "Profile",
    icon: "heroicons:user-circle",
    desc: "Your personal info",
  },
  {
    key: "security",
    label: "Security",
    icon: "heroicons:lock-closed",
    desc: "Password & access",
  },
  {
    key: "notifications",
    label: "Notifications",
    icon: "heroicons:bell",
    desc: "Email preferences",
  },
];


const profileSchema = v.object({
  firstName: v.pipe(v.string(), v.minLength(1, "First name is required.")),
  lastName: v.pipe(v.string(), v.minLength(1, "Last name is required.")),
  username: v.pipe(
    v.string(),
    v.minLength(2, "Username must be at least 2 characters."),
  ),
  email: v.pipe(v.string(), v.email("Please enter a valid email address.")),
});

type ProfileSchema = v.InferOutput<typeof profileSchema>;

const profileState = reactive<ProfileSchema>({
  firstName: userStore.user.firstName ?? "",
  lastName: userStore.user.lastName ?? "",
  username: userStore.user.username ?? "",
  email: userStore.user.email ?? "",
});

const profileLoading = ref(false);

const initials = computed(() => useGetInitials(userStore.user));

async function onProfileSubmit({ data }: FormSubmitEvent<ProfileSchema>) {
  profileLoading.value = true;
  try {
    userStore.updateProfile({ ...userStore.user, ...data });
    toast.add({ title: "Profile updated", color: "success" });
  } catch {
    toast.add({ title: "Failed to update profile", color: "error" });
  } finally {
    profileLoading.value = false;
  }
}


const passwordSchema = v.pipe(
  v.object({
    currentPassword: v.pipe(
      v.string(),
      v.minLength(1, "Current password is required."),
    ),
    newPassword: v.pipe(
      v.string(),
      v.minLength(8, "Password must be at least 8 characters."),
    ),
    confirmPassword: v.pipe(
      v.string(),
      v.minLength(1, "Please confirm your password."),
    ),
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
  newPassword: "",
  confirmPassword: "",
});

const passwordLoading = ref(false);

async function onPasswordSubmit({ data: _ }: FormSubmitEvent<PasswordSchema>) {
  passwordLoading.value = true;
  try {
    toast.add({ title: "Password changed", color: "success" });
    passwordState.currentPassword = "";
    passwordState.newPassword = "";
    passwordState.confirmPassword = "";
  } catch {
    toast.add({ title: "Failed to change password", color: "error" });
  } finally {
    passwordLoading.value = false;
  }
}


const notificationItems = [
  {
    key: "productUpdates",
    label: "Product updates",
    desc: "News and feature announcements.",
    icon: "heroicons:megaphone",
  },
  {
    key: "teamActivity",
    label: "Team activity",
    desc: "When members join, leave, or change roles.",
    icon: "heroicons:users",
  },
  {
    key: "marketplaceAlerts",
    label: "Marketplace alerts",
    desc: "Stock, pricing, and listing changes.",
    icon: "heroicons:building-storefront",
  },
  {
    key: "weeklyDigest",
    label: "Weekly digest",
    desc: "A summary of workspace activity.",
    icon: "heroicons:newspaper",
  },
  {
    key: "securityAlerts",
    label: "Security alerts",
    desc: "Sign-ins from new devices or locations.",
    icon: "heroicons:shield-check",
  },
];

const notifications = reactive({
  productUpdates: true,
  teamActivity: true,
  marketplaceAlerts: false,
  weeklyDigest: true,
  securityAlerts: true,
});

function saveNotifications() {
  toast.add({ title: "Notification preferences saved", color: "success" });
}
</script>

<template>
  <div class="flex gap-6 items-start">
    <!-- ── Left nav  -->
    <aside class="w-52 shrink-0">
      <div
        class="bg-white dark:bg-brand-dark-600 border border-gray-100 dark:border-white/5 rounded-2xl p-2 sticky top-0"
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
            class="flex items-center gap-3 w-full px-3 py-2.5 rounded-xl text-sm transition-all text-left"
            :class="
              activeTab === tab.key
                ? 'bg-brand text-white font-medium'
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
                    ? 'text-white/70'
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

    <!-- ── Content ─ -->
    <div class="flex-1 min-w-0 max-w-xl">
      <!-- Profile ── -->
      <template v-if="activeTab === 'profile'">
        <div class="space-y-4">
          <!-- Avatar card -->
          <div
            class="bg-white dark:bg-brand-dark-600 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
          >
            <div class="flex items-center gap-4">
              <div
                class="w-14 h-14 rounded-2xl bg-brand-50 dark:bg-brand/10 flex items-center justify-center shrink-0"
              >
                <span class="text-lg font-bold text-brand">{{ initials }}</span>
              </div>
              <div>
                <p class="font-semibold text-gray-900 dark:text-white">
                  {{
                    [profileState.firstName, profileState.lastName]
                      .filter(Boolean)
                      .join(" ") || "—"
                  }}
                </p>
                <p class="text-sm text-gray-400 dark:text-white/30 mt-0.5">
                  @{{ profileState.username || "username" }}
                </p>
              </div>
            </div>
          </div>

          <!-- Form card -->
          <div
            class="bg-white dark:bg-brand-dark-600 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
          >
            <p class="font-semibold text-gray-900 dark:text-white mb-1">
              Personal information
            </p>
            <p class="text-xs text-gray-400 dark:text-white/30 mb-5">
              Update your name, username, and email.
            </p>

            <UForm
              :schema="profileSchema"
              :state="profileState"
              class="space-y-4"
              :on-submit="onProfileSubmit"
            >
              <div class="grid grid-cols-2 gap-4">
                <AppInput
                  v-model="profileState.firstName"
                  label="First name"
                  name="firstName"
                  placeholder="Jane"
                />

                <AppInput
                  v-model="profileState.lastName"
                  label="Last name"
                  name="lastName"
                  placeholder="Doe"
                />
              </div>

              <AppInput
                v-model="profileState.username"
                label="Username"
                name="username"
                placeholder="janedoe"
              />

              <AppInput
                v-model="profileState.email"
                label="Email address"
                name="email"
                placeholder="jane@example.com"
              />

              <div class="pt-1 flex items-center justify-between">
                <AppButton
                  type="submit"
                  :loading="profileLoading"
                  :disabled="profileLoading"
                >
                  Save changes
                </AppButton>
              </div>
            </UForm>
          </div>
        </div>
      </template>

      <!-- Security ── -->
      <template v-else-if="activeTab === 'security'">
        <div class="space-y-4">
          <!-- Info banner -->
          <div
            class="bg-brand-50 dark:bg-brand/10 border border-brand-100 dark:border-brand/20 rounded-2xl p-4 flex items-start gap-3"
          >
            <UIcon
              name="heroicons:information-circle"
              class="size-4 text-brand shrink-0 mt-0.5"
            />
            <p
              class="text-xs text-brand-600 dark:text-brand-300 leading-relaxed"
            >
              Use a strong, unique password at least 8 characters long. Avoid
              reusing passwords from other sites.
            </p>
          </div>

          <!-- Form card -->
          <div
            class="bg-white dark:bg-brand-dark-600 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
          >
            <p class="font-semibold text-gray-900 dark:text-white mb-1">
              Change password
            </p>
            <p class="text-xs text-gray-400 dark:text-white/30 mb-5">
              You'll be asked to sign in again after changing your password.
            </p>

            <UForm
              :schema="passwordSchema"
              :state="passwordState"
              class="space-y-4"
              :on-submit="onPasswordSubmit"
            >
              <AppInput
                v-model="passwordState.currentPassword"
                label="Current password"
                name="currentPassword"
                type="password"
                placeholder="••••••••"
              />

              <div
                class="border-t border-gray-100 dark:border-white/5 pt-4 space-y-4"
              >
                <AppInput
                  v-model="passwordState.newPassword"
                  label="New password"
                  name="newPassword"
                  type="password"
                  placeholder="••••••••"
                />

                <AppInput
                  v-model="passwordState.confirmPassword"
                  label="Confirm new password"
                  name="confirmPassword"
                  type="password"
                  placeholder="••••••••"
                />
              </div>

              <div class="pt-1">
                <AppButton
                  type="submit"
                  :loading="passwordLoading"
                  :disabled="passwordLoading"
                >
                  Change password
                </AppButton>
              </div>
            </UForm>
          </div>
        </div>
      </template>

      <!-- Notifications ── -->
      <template v-else-if="activeTab === 'notifications'">
        <div
          class="bg-white dark:bg-brand-dark-600 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
        >
          <p class="font-semibold text-gray-900 dark:text-white mb-1">
            Email notifications
          </p>
          <p class="text-xs text-gray-400 dark:text-white/30 mb-5">
            Choose which emails you'd like to receive.
          </p>

          <div class="space-y-2">
            <div
              v-for="item in notificationItems"
              :key="item.key"
              class="flex items-center gap-4 px-4 py-3.5 rounded-xl border border-gray-100 dark:border-white/5 hover:bg-gray-50 dark:hover:bg-white/[0.02] transition-colors"
            >
              <div
                class="w-8 h-8 rounded-lg bg-gray-50 dark:bg-white/5 flex items-center justify-center shrink-0"
              >
                <UIcon
                  :name="item.icon"
                  class="size-4 text-gray-400 dark:text-white/25"
                />
              </div>
              <div class="flex-1 min-w-0">
                <p class="text-sm font-medium text-gray-800 dark:text-white/80">
                  {{ item.label }}
                </p>
                <p class="text-xs text-gray-400 dark:text-white/30 mt-0.5">
                  {{ item.desc }}
                </p>
              </div>
              <UToggle v-model="(notifications as any)[item.key]" />
            </div>
          </div>

          <div class="mt-5 pt-4 border-t border-gray-100 dark:border-white/5">
            <AppButton @click="saveNotifications"> Save preferences </AppButton>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped></style>
