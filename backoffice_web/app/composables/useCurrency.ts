import { useAppStore } from "~/stores/app";
import { useCountryStore } from "~/stores/country";

const FALLBACK_CURRENCY = "NGN";

export function useCurrency() {
  const appStore = useAppStore();
  const countryStore = useCountryStore();

  const defaultCountry = computed(() => {
    const value = appStore.config?.defaultCurrency;
    if (!value) return null;
    const needle = value.toLowerCase();
    return (
      countryStore.countries.find(
        (c) =>
          c.identifier.toLowerCase() === needle ||
          c.currencyCode.toLowerCase() === needle,
      ) ?? null
    );
  });

  const currencyCode = computed(
    () => defaultCountry.value?.currencyCode ?? FALLBACK_CURRENCY,
  );

  function formatMoney(
    amount: number | bigint | string | null | undefined,
  ): string {
    const value =
      amount === null || amount === undefined
        ? 0
        : Number(typeof amount === "bigint" ? amount : amount);
    if (Number.isNaN(value)) {
      return new Intl.NumberFormat(undefined, {
        style: "currency",
        currency: currencyCode.value,
        minimumFractionDigits: 0,
        maximumFractionDigits: 2,
      }).format(0);
    }
    return new Intl.NumberFormat(undefined, {
      style: "currency",
      currency: currencyCode.value,
      minimumFractionDigits: 0,
      maximumFractionDigits: 2,
    }).format(value);
  }

  async function ensureLoaded() {
    const tasks: Promise<unknown>[] = [];
    if (!appStore.config) tasks.push(appStore.fetchConfig());
    if (countryStore.countries.length === 0)
      tasks.push(countryStore.fetchCountries());
    if (tasks.length > 0) await Promise.all(tasks);
  }

  return {
    defaultCountry,
    currencyCode,
    formatMoney,
    ensureLoaded,
  };
}
