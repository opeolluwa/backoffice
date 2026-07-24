interface RGB {
  r: number;
  g: number;
  b: number;
}

interface OKLCH {
  l: number;
  c: number;
  h: number;
}

export type PaletteShades = Record<string, string>;

function hexToRgb(hex: string): RGB {
  const h = hex.replace("#", "");
  return {
    r: parseInt(h.substring(0, 2), 16) / 255,
    g: parseInt(h.substring(2, 4), 16) / 255,
    b: parseInt(h.substring(4, 6), 16) / 255,
  };
}

function rgbToHex(rgb: RGB): string {
  const toHex = (v: number) => {
    const clamped = Math.max(0, Math.min(1, v));
    const hex = Math.round(clamped * 255)
      .toString(16)
      .padStart(2, "0");
    return hex;
  };
  return `#${toHex(rgb.r)}${toHex(rgb.g)}${toHex(rgb.b)}`;
}

export function hexToRgbString(hex: string): string {
  const rgb = hexToRgb(hex);
  const r = Math.round(rgb.r * 255);
  const g = Math.round(rgb.g * 255);
  const b = Math.round(rgb.b * 255);
  return `${r} ${g} ${b}`;
}

function linearize(c: number): number {
  return c <= 0.04045 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);
}

function delinearize(c: number): number {
  const abs = Math.abs(c);
  if (abs <= 0.0031308) return c * 12.92;
  return (1.0 + 0.055) * Math.pow(abs, 1.0 / 2.4) * Math.sign(c);
}

function rgbToOklch(rgb: RGB): OKLCH {
  const lr = linearize(rgb.r);
  const lg = linearize(rgb.g);
  const lb = linearize(rgb.b);

  const l_ = 0.4122214708 * lr + 0.5363325363 * lg + 0.0514459929 * lb;
  const m_ = 0.2119034982 * lr + 0.6806995451 * lg + 0.1073969566 * lb;
  const s_ = 0.0883024619 * lr + 0.2817188376 * lg + 0.6299787005 * lb;

  const l = Math.sign(l_) * Math.pow(Math.abs(l_), 1 / 3);
  const m = Math.sign(m_) * Math.pow(Math.abs(m_), 1 / 3);
  const s = Math.sign(s_) * Math.pow(Math.abs(s_), 1 / 3);

  const L = 0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s;
  const a = 1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s;
  const b = 0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s;

  const c = Math.sqrt(a * a + b * b);
  let h = (Math.atan2(b, a) * 180) / Math.PI;
  if (h < 0) h += 360;

  return { l: L, c, h };
}

function oklchToRgb(oklch: OKLCH): RGB {
  const { l, c, h } = oklch;
  const hRad = (h * Math.PI) / 180;

  const a = c * Math.cos(hRad);
  const b = c * Math.sin(hRad);

  const l_ = l + 0.3963377774 * a + 0.2158037573 * b;
  const m_ = l - 0.1055613458 * a - 0.0638541728 * b;
  const s_ = l - 0.0894841775 * a - 1.291485548 * b;

  const l3 = l_ * l_ * l_;
  const m3 = m_ * m_ * m_;
  const s3 = s_ * s_ * s_;

  const lr = +4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3;
  const lg = -1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3;
  const lb = -0.0041960863 * l3 - 0.7034186147 * m3 + 1.707614701 * s3;

  return {
    r: delinearize(lr),
    g: delinearize(lg),
    b: delinearize(lb),
  };
}

function oklchToHex(oklch: OKLCH): string {
  return rgbToHex(oklchToRgb(oklch));
}

const SHADE_MAP: Record<string, { l: number; cScale: number }> = {
  "50": { l: 0.97, cScale: 0.01 },
  "100": { l: 0.92, cScale: 0.04 },
  "200": { l: 0.85, cScale: 0.08 },
  "300": { l: 0.76, cScale: 0.12 },
  "400": { l: 0.66, cScale: 0.16 },
  "500": { l: 0.55, cScale: 0.19 },
  "600": { l: 0.47, cScale: 0.17 },
  "700": { l: 0.39, cScale: 0.14 },
  "800": { l: 0.31, cScale: 0.11 },
  "900": { l: 0.24, cScale: 0.08 },
  "950": { l: 0.17, cScale: 0.05 },
};

export function generatePalette(hex: string): PaletteShades {
  const rgb = hexToRgb(hex);
  const oklch = rgbToOklch(rgb);
  const palette: PaletteShades = {};

  for (const [shade, target] of Object.entries(SHADE_MAP)) {
    palette[shade] = oklchToHex({
      l: target.l,
      c: oklch.c * target.cScale,
      h: oklch.h,
    });
  }

  return palette;
}

export function generateDarkPalette(hex: string): PaletteShades {
  const rgb = hexToRgb(hex);
  const oklch = rgbToOklch(rgb);
  const palette: PaletteShades = {};

  const darkShades: Record<string, { l: number; cScale: number }> = {
    "50": { l: 0.28, cScale: 0.06 },
    "100": { l: 0.25, cScale: 0.055 },
    "200": { l: 0.22, cScale: 0.05 },
    "300": { l: 0.19, cScale: 0.045 },
    "400": { l: 0.16, cScale: 0.04 },
    "500": { l: 0.14, cScale: 0.035 },
    "600": { l: 0.12, cScale: 0.03 },
    "700": { l: 0.1, cScale: 0.025 },
    "800": { l: 0.08, cScale: 0.02 },
    "900": { l: 0.06, cScale: 0.015 },
    "950": { l: 0.04, cScale: 0.01 },
  };

  for (const [shade, target] of Object.entries(darkShades)) {
    palette[shade] = oklchToHex({
      l: target.l,
      c: oklch.c * target.cScale,
      h: oklch.h,
    });
  }

  return palette;
}

export function paletteToCssVars(
  palette: PaletteShades,
  prefix: string,
): string {
  return Object.entries(palette)
    .map(
      ([shade, color]) =>
        `  --color-${prefix}-${shade}: rgb(${hexToRgbString(color)});\n  --ui-${prefix}-${shade}: rgb(${hexToRgbString(color)});`,
    )
    .join("\n");
}
