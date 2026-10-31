import type { Locale } from '../core/locale';
import { da } from './da';
import { de } from './de';
import { bavarian, berlin, cologne, hamburgPlatt, hessian, swabian, swissGerman } from './dialects';
import { badish, eastFrisian, franconian, koblenz, palatinate, ruhr, saarland, saxon, viennese } from './dialects-more';
import { el } from './el';
import { en } from './en';
import { es } from './es';
import { formrauschLocale } from './formrausch';
import { fr } from './fr';
import { it } from './it';
import { ja } from './ja';
import { nl } from './nl';
import { sv } from './sv';

/** Languages grouped for the picker; the order here is also the arrow-key order. */
export const LOCALE_GROUPS: readonly { name: string; note: string; locales: readonly Locale[] }[] = [
  { name: 'Deutsch', note: 'Hochdeutsch und das Formrausch-Raster', locales: [de, formrauschLocale] },
  {
    name: 'Dialekte',
    note: 'Eigene Raster mit einem Wort aus der Gegend in den Füllbuchstaben',
    locales: [
      swabian,
      badish,
      bavarian,
      franconian,
      viennese,
      swissGerman,
      saxon,
      berlin,
      hamburgPlatt,
      eastFrisian,
      ruhr,
      cologne,
      koblenz,
      hessian,
      palatinate,
      saarland,
    ],
  },
  { name: 'Europa', note: 'Eigene Raster', locales: [en, nl, da, sv, fr, es, it, el] },
  { name: 'Asien', note: 'Quadratische Schriftzeichen', locales: [ja] },
];

export const LOCALES: readonly Locale[] = LOCALE_GROUPS.flatMap((group) => group.locales);

export function localeById(id: string): Locale {
  return LOCALES.find((l) => l.id === id) ?? de;
}
