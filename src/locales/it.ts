import { createGrid, type Cells } from '../core/grid';
import type { Locale } from '../core/locale';

const grid = createGrid([
  "SONOÈLEL'BS",
  'UNADUETRERS',
  'QUATTROSEIA',
  'CINQUESETTE',
  'OTTONOVEZAB',
  'DIECIUNDICI',
  'DODICIEMENO',
  'UNQUARTOVLA',
  'VENTICINQUE',
  'DIECIRMEZZA',
]);

const w = {
  sono: grid.word(0, 0, 'SONO'),
  e: grid.word(0, 4, 'È'),
  le: grid.word(0, 5, 'LE'),
  l: grid.word(0, 7, "L'"),
  and: grid.word(6, 6, 'E'),
  meno: grid.word(6, 7, 'MENO'),
  un: grid.word(7, 0, 'UN'),
  quarto: grid.word(7, 2, 'QUARTO'),
  venti: grid.word(8, 0, 'VENTI'),
  venticinque: grid.word(8, 0, 'VENTICINQUE'),
  cinque: grid.word(8, 5, 'CINQUE'),
  dieci: grid.word(9, 0, 'DIECI'),
  mezza: grid.word(9, 6, 'MEZZA'),
};

// Index = hour % 12, so index 0 is DODICI.
const hours: Cells[] = [
  grid.word(6, 0, 'DODICI'),
  grid.word(1, 0, 'UNA'),
  grid.word(1, 3, 'DUE'),
  grid.word(1, 6, 'TRE'),
  grid.word(2, 0, 'QUATTRO'),
  grid.word(3, 0, 'CINQUE'),
  grid.word(2, 7, 'SEI'),
  grid.word(3, 6, 'SETTE'),
  grid.word(4, 0, 'OTTO'),
  grid.word(4, 4, 'NOVE'),
  grid.word(5, 0, 'DIECI'),
  grid.word(5, 5, 'UNDICI'),
];

const minutes: Cells[][] = [
  [],
  [w.and, w.cinque],
  [w.and, w.dieci],
  [w.and, w.un, w.quarto],
  [w.and, w.venti],
  [w.and, w.venticinque],
  [w.and, w.mezza],
  [w.meno, w.venticinque],
  [w.meno, w.venti],
  [w.meno, w.un, w.quarto],
  [w.meno, w.dieci],
  [w.meno, w.cinque],
];

export const it: Locale = {
  id: 'it',
  name: 'Italienisch',
  lang: 'it-IT',
  grid,
  words: [...Object.values(w), ...hours],
  frame(now, { intro }) {
    const step = Math.floor(now.getMinutes() / 5);
    const h = (step >= 7 ? now.getHours() + 1 : now.getHours()) % 12;
    // "È l'una", but "sono le due".
    const intros = intro ? (h === 1 ? [w.e, w.l] : [w.sono, w.le]) : [];
    return { words: [...intros, hours[h]!, ...minutes[step]!], dots: now.getMinutes() % 5 };
  },
};
