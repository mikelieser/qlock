import { LOCALES } from './locales';

/** Themes grouped by whose they are; the order here is also the arrow-key order. */
export const THEME_GROUPS = [
  {
    name: 'Standard',
    note: 'Die Klassiker aus dem alten Stand',
    themes: ['standard', 'formrausch', 'graphite', 'paper', 'brass', 'sage', 'blue', 'red', 'metal', 'gradient', 'ticktack', 'timeonly'],
  },
  {
    name: 'Mike',
    note: 'Strand, Sonne, Retro',
    themes: [
      // Morning, day, evening, night: the order "Tageslauf" goes through.
      'sunrise',
      'lagoon',
      'sunset',
      'nightsurf',
      'dunes',
      'retro',
      'retro-sun',
      'retro-stripes',
      'retro-night',
      'retro-pastel',
    ],
  },
  {
    name: 'Susanna',
    note: 'Pastell, diagonal von oben links',
    themes: ['cotton-candy', 'lavender', 'bubblegum', 'sky-dream', 'unicorn'],
  },
  {
    name: 'Henry',
    note: 'Chaos',
    themes: ['chaos', 'chaos-burst', 'chaos-night'],
  },
  {
    name: 'Katja',
    note: 'Herbst und gemütliche Abende',
    themes: ['autumn-leaves', 'golden-hour', 'forest-mist', 'cozy'],
  },
  {
    name: 'Feste',
    note: 'Motto-Themes durchs Jahr',
    themes: ['karneval', 'ostern', 'muttertag', 'vatertag', 'halloween', 'nikolaus', 'weihnachten', 'silvester'],
  },
] as const;

/** Themes that were renamed; saved settings and old links still work. */
const RENAMED: Record<string, Theme> = { katja: 'halloween', neujahr: 'silvester' };

export type Theme = (typeof THEME_GROUPS)[number]['themes'][number];
export const THEMES: readonly Theme[] = THEME_GROUPS.flatMap((group) => group.themes);

export const themeLabel = (theme: Theme): string =>
  theme
    .split('-')
    .map((part) => part[0]!.toUpperCase() + part.slice(1))
    .join(' ');
export const MODES = ['live', 'demo', 'all'] as const;

export type Mode = (typeof MODES)[number];

export interface Settings {
  locale: string;
  theme: Theme;
  mode: Mode;
  intro: boolean;
  dots: boolean;
  stretch: boolean;
  progress: boolean;
  date: boolean;
  effect: boolean;
  transition: boolean;
  ambient: boolean;
  /** Switch to the festive theme around Karneval, Ostern, Halloween, Advent, and Silvester. */
  festive: boolean;
  /** Theme follows the sun: sunrise, lagoon, sunset, nightsurf. */
  daylight: boolean;
}

export const DEFAULTS: Settings = {
  locale: 'de',
  theme: 'formrausch',
  mode: 'live',
  intro: true,
  dots: true,
  stretch: false,
  progress: false,
  date: false,
  effect: false,
  transition: false,
  ambient: false,
  festive: false,
  daylight: false,
};
const STORAGE_KEY = 'qlock:settings';

const oneOf = <T extends string>(list: readonly T[], value: unknown): T | undefined =>
  list.find((item) => item === value);

const bool = (value: unknown): boolean | undefined => {
  if (typeof value === 'boolean') return value;
  if (value === '1' || value === 'true') return true;
  if (value === '0' || value === 'false') return false;
  return undefined;
};

function sanitize(raw: Record<string, unknown>): Partial<Settings> {
  const parsed: Partial<Settings> = {
    locale: oneOf(
      LOCALES.map((l) => l.id),
      raw.locale,
    ),
    theme: oneOf(THEMES, typeof raw.theme === 'string' ? (RENAMED[raw.theme] ?? raw.theme) : raw.theme),
    mode: oneOf(MODES, raw.mode),
    intro: bool(raw.intro),
    dots: bool(raw.dots),
    stretch: bool(raw.stretch),
    progress: bool(raw.progress),
    date: bool(raw.date),
    effect: bool(raw.effect),
    transition: bool(raw.transition),
    ambient: bool(raw.ambient),
    festive: bool(raw.festive),
    daylight: bool(raw.daylight),
  };
  return Object.fromEntries(Object.entries(parsed).filter(([, v]) => v !== undefined));
}

/** Defaults, overridden by the last saved settings, overridden by URL params (?locale=en&theme=metal). */
export function loadSettings(): Settings {
  let stored: Record<string, unknown> = {};
  try {
    stored = JSON.parse(localStorage.getItem(STORAGE_KEY) ?? '{}');
  } catch {
    // Private mode or broken JSON: start from defaults.
  }
  const params = Object.fromEntries(new URLSearchParams(location.search));
  return { ...DEFAULTS, ...sanitize(stored), ...sanitize(params) };
}

export function saveSettings(settings: Settings): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  } catch {
    // Not persisting is fine.
  }
}

export function cycle<T>(list: readonly T[], current: T, step = 1): T {
  const n = list.length;
  return list[(((list.indexOf(current) + step) % n) + n) % n]!;
}
