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
import { amt, anhalter, bahn, boerse, derb, digga, familie, fussball, kids, nerd, pirat, zen } from './fun';
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
  { name: 'Spaß', note: 'Augenzwinkernde Blätter auf Deutsch', locales: [familie, kids, digga, boerse, nerd, anhalter, bahn, amt, pirat, fussball, zen, derb] },
  { name: 'Sprachen', note: 'Eigene Raster, Japanisch mit quadratischen Schriftzeichen', locales: [en, nl, da, sv, fr, es, it, el, ja] },
];

export const LOCALES: readonly Locale[] = LOCALE_GROUPS.flatMap((group) => group.locales);

export function localeById(id: string): Locale {
  return LOCALES.find((l) => l.id === id) ?? de;
}
