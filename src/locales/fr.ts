import { createGrid, type Cells } from '../core/grid';
import type { Locale } from '../core/locale';

const grid = createGrid([
  'ILNESTODEUX',
  'QUATRETROIS',
  'NEUFUNESEPT',
  'HUITSIXCINQ',
  'MIDIXMINUIT',
  'ONZERHEURES',
  'MOINSOLEDIX',
  'ETRQUARTRED',
  'VINGT-CINQU',
  'ETSDEMIEPAM',
]);

const w = {
  il: grid.word(0, 0, 'IL'),
  est: grid.word(0, 3, 'EST'),
  heure: grid.word(5, 5, 'HEURE'),
  heures: grid.word(5, 5, 'HEURES'),
  moins: grid.word(6, 0, 'MOINS'),
  le: grid.word(6, 6, 'LE'),
  dix: grid.word(6, 8, 'DIX'),
  et: grid.word(7, 0, 'ET'),
  quart: grid.word(7, 3, 'QUART'),
  vingt: grid.word(8, 0, 'VINGT'),
  vingtCinq: grid.word(8, 0, 'VINGT-CINQ'),
  cinq: grid.word(8, 6, 'CINQ'),
  etHalf: grid.word(9, 0, 'ET'),
  demie: grid.word(9, 3, 'DEMIE'),
};

// Index = hour of the day % 24 mapped to 12 words; 0 is MINUIT, 12 is MIDI (no HEURES after them).
const hourWords: Record<number, Cells> = {
  1: grid.word(2, 4, 'UNE'),
  2: grid.word(0, 7, 'DEUX'),
  3: grid.word(1, 6, 'TROIS'),
  4: grid.word(1, 0, 'QUATRE'),
  5: grid.word(3, 7, 'CINQ'),
  6: grid.word(3, 4, 'SIX'),
  7: grid.word(2, 7, 'SEPT'),
  8: grid.word(3, 0, 'HUIT'),
  9: grid.word(2, 0, 'NEUF'),
  10: grid.word(4, 2, 'DIX'),
  11: grid.word(5, 0, 'ONZE'),
};
const midi = grid.word(4, 0, 'MIDI');
const minuit = grid.word(4, 5, 'MINUIT');

/** "DEUX HEURES", "UNE HEURE", "MIDI", "MINUIT" for an hour of the day (0–23). */
function hourPhrase(hour24: number): Cells[] {
  const h = hour24 % 24;
  if (h === 0) return [minuit];
  if (h === 12) return [midi];
  const h12 = h % 12;
  return [hourWords[h12]!, h12 === 1 ? w.heure : w.heures];
}

const minutes: Cells[][] = [
  [],
  [w.cinq],
  [w.dix],
  [w.et, w.quart],
  [w.vingt],
  [w.vingtCinq],
  [w.etHalf, w.demie],
  [w.moins, w.vingtCinq],
  [w.moins, w.vingt],
  [w.moins, w.le, w.quart],
  [w.moins, w.dix],
  [w.moins, w.cinq],
];

export const fr: Locale = {
  id: 'fr',
  name: 'Französisch',
  lang: 'fr-FR',
  grid,
  words: [w.il, w.est, ...Object.values(hourWords), midi, minuit, w.heures, w.moins, w.le, w.dix, w.et, w.quart, w.vingtCinq, w.etHalf, w.demie],
  frame(now, { intro }) {
    const step = Math.floor(now.getMinutes() / 5);
    // From twenty-five to on, French counts down to the next hour.
    const hour = step >= 7 ? now.getHours() + 1 : now.getHours();
    const intros = intro ? [w.il, w.est] : [];
    return { words: [...intros, ...hourPhrase(hour), ...minutes[step]!], dots: now.getMinutes() % 5 };
  },
};
