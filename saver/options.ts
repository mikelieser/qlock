// Writes the screensaver's option lists (languages, themes, switches) as JSON, so the
// Swift options sheet never needs a change when a theme or dialect is added.
// Usage: bun saver/options.ts <out.json>
import { LOCALE_GROUPS } from '../src/locales';
import { DEFAULTS, THEME_GROUPS, themeLabel, type Settings } from '../src/settings';

type Switch = { [K in keyof Settings]: Settings[K] extends boolean ? K : never }[keyof Settings];

// Grouped for the options sheet: what is shown, and what moves.
const SWITCHES: { key: Switch; label: string; group: 'Anzeige' | 'Bewegung' }[] = [
  { key: 'intro', label: '„Es ist“', group: 'Anzeige' },
  { key: 'dots', label: 'Minutenpunkte', group: 'Anzeige' },
  { key: 'date', label: 'Wochentag und Datum', group: 'Anzeige' },
  { key: 'progress', label: 'Sekundenrahmen', group: 'Anzeige' },
  { key: 'stretch', label: 'Ganzer Bildschirm', group: 'Anzeige' },
  { key: 'effect', label: 'Buchstaben', group: 'Bewegung' },
  { key: 'transition', label: 'Zeitwechsel', group: 'Bewegung' },
  { key: 'ambient', label: 'Hintergrund', group: 'Bewegung' },
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
