import {
  generatePalette,
  generateDarkPalette,
  paletteToCssVars,
} from "~/utils/color";

const BRAND_COLOR_KEY = "brand_color";
const PALETTE_CACHE_KEY = "brand_palette_cache";

function getCachedBrandColor(): string | null {
  if (import.meta.server) return null;
  return localStorage.getItem(BRAND_COLOR_KEY);
}

function setCachedBrandColor(hex: string): void {
  if (import.meta.server) return;
  localStorage.setItem(BRAND_COLOR_KEY, hex);
}

function getCachedPalette(): { light: string; dark: string } | null {
  if (import.meta.server) return null;
  const raw = localStorage.getItem(PALETTE_CACHE_KEY);
  if (!raw) return null;
  try {
    return JSON.parse(raw);
  } catch {
    return null;
  }
}

function setCachedPalette(light: string, dark: string): void {
  if (import.meta.server) return;
  localStorage.setItem(PALETTE_CACHE_KEY, JSON.stringify({ light, dark }));
}

function injectPalette(lightVars: string, darkVars: string): void {
  if (import.meta.server) return;

  let styleEl = document.getElementById("brand-colors") as HTMLStyleElement;
  if (!styleEl) {
    styleEl = document.createElement("style");
    styleEl.id = "brand-colors";
    document.head.appendChild(styleEl);
  }

  styleEl.textContent = `:root {${lightVars}} .dark {${darkVars}}`;
}

function removePalette(): void {
  if (import.meta.server) return;
  const el = document.getElementById("brand-colors");
  if (el) el.remove();
}

export function useBrandColor() {
  const appStore = useAppStore();

  function applyBrandColor(hex: string): void {
    if (!hex || !/^#[0-9A-Fa-f]{6}$/.test(hex)) {
      removePalette();
      return;
    }

    const lightPalette = generatePalette(hex);
    const darkPalette = generateDarkPalette(hex);

    const lightVars = paletteToCssVars(lightPalette, "primary");
    const darkVars = paletteToCssVars(darkPalette, "primary");

    injectPalette(lightVars, darkVars);
    setCachedBrandColor(hex);
    setCachedPalette(lightVars, darkVars);
  }

  function applyFromCache(): boolean {
    const cached = getCachedBrandColor();
    if (!cached) return false;

    const paletteCache = getCachedPalette();
    if (paletteCache) {
      injectPalette(paletteCache.light, paletteCache.dark);
      return true;
    }

    applyBrandColor(cached);
    return true;
  }

  async function initBrandColor(): Promise<void> {
    if (import.meta.server) return;

    applyFromCache();

    try {
      if (!appStore.config) {
        await appStore.fetchConfig();
      }

      const brandColor = appStore.config?.brandColor;
      if (brandColor) {
        applyBrandColor(brandColor);
      }
    } catch {
      // Cache already applied, graceful fallback
    }
  }

  function clearBrandColor(): void {
    removePalette();
    if (import.meta.server) return;
    localStorage.removeItem(BRAND_COLOR_KEY);
    localStorage.removeItem(PALETTE_CACHE_KEY);
  }

  return {
    applyBrandColor,
    initBrandColor,
    clearBrandColor,
  };
}
