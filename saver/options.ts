// Writes the screensaver's option lists (languages, themes, switches) as JSON, so the
// Swift options sheet never needs a change when a theme or dialect is added.
// Usage: bun saver/options.ts <out.json>
import { LOCALE_GROUPS } from '../src/locales';
import { DEFAULTS, THEME_GROUPS, themeLabel, type Settings } from '../src/settings';

type Switch = { [K in keyof Settings]: Settings[K] extends boolean ? K : never }[keyof Settings];

const SWITCHES: { key: Switch; label: string }[] = [
  { key: 'date', label: 'Wochentag und Datum' },
  { key: 'effect', label: 'Buchstaben-Effekt' },
  { key: 'transition', label: 'Übergang beim Zeitwechsel' },
  { key: 'ambient', label: 'Hintergrund-Effekt' },
  { key: 'progress', label: 'Sekundenrahmen' },
  { key: 'dots', label: 'Minutenpunkte' },
  { key: 'intro', label: '„Es ist“ anzeigen' },
  { key: 'stretch', label: 'Ganzen Bildschirm füllen' },
];

const options = {
  locales: LOCALE_GROUPS.map((group) => ({
    group: group.name,
    items: group.locales.map((locale) => ({ id: locale.id, label: locale.name, lang: locale.lang })),
  })),
  themes: THEME_GROUPS.map((group) => ({
    group: group.name,
    items: group.themes.map((theme) => ({ id: theme, label: themeLabel(theme) })),
  })),
  switches: SWITCHES.map((s) => ({ ...s, default: DEFAULTS[s.key] })),
  defaults: { locale: DEFAULTS.locale, theme: DEFAULTS.theme },
};

const out = process.argv[2];
if (!out) throw new Error('usage: bun saver/options.ts <out.json>');
await Bun.write(out, JSON.stringify(options, null, 2));
