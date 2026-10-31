import { createGrid, type Cells } from '../core/grid';
import type { Locale } from '../core/locale';

// Greek drops accents in capitals, so ΕΙΝΑΙ, ΜΙΑ, ΜΙΣΗ need no special cells.
const grid = createGrid([
  'ΕΙΝΑΙΘΜΙΑΖΞ',
  'ΔΥΟΤΡΕΙΣΕΞΙ',
  'ΤΕΣΣΕΡΙΣΨΦΩ',
  'ΠΕΝΤΕΕΠΤΑΛΓ',
  'ΟΚΤΩΕΝΝΕΑΒΡ',
  'ΔΕΚΑΕΝΤΕΚΑΥ',
  'ΔΩΔΕΚΑΣΩΛΗΖ',
  'ΚΑΙΠΑΡΑΔΕΚΑ',
  'ΕΙΚΟΣΙΠΕΝΤΕ',
  'ΤΕΤΑΡΤΟΜΙΣΗ',
]);

const w = {
  einai: grid.word(0, 0, 'ΕΙΝΑΙ'),
  kai: grid.word(7, 0, 'ΚΑΙ'),
  para: grid.word(7, 3, 'ΠΑΡΑ'),
  deka: grid.word(7, 7, 'ΔΕΚΑ'),
  eikosi: grid.word(8, 0, 'ΕΙΚΟΣΙ'),
  eikosipente: grid.word(8, 0, 'ΕΙΚΟΣΙΠΕΝΤΕ'),
  pente: grid.word(8, 6, 'ΠΕΝΤΕ'),
  tetarto: grid.word(9, 0, 'ΤΕΤΑΡΤΟ'),
  misi: grid.word(9, 7, 'ΜΙΣΗ'),
};

// Index = hour % 12, so index 0 is ΔΩΔΕΚΑ. One, three and four take the feminine form (η ώρα).
const hours: Cells[] = [
  grid.word(6, 0, 'ΔΩΔΕΚΑ'),
  grid.word(0, 6, 'ΜΙΑ'),
  grid.word(1, 0, 'ΔΥΟ'),
  grid.word(1, 3, 'ΤΡΕΙΣ'),
  grid.word(2, 0, 'ΤΕΣΣΕΡΙΣ'),
  grid.word(3, 0, 'ΠΕΝΤΕ'),
  grid.word(1, 8, 'ΕΞΙ'),
  grid.word(3, 5, 'ΕΠΤΑ'),
  grid.word(4, 0, 'ΟΚΤΩ'),
  grid.word(4, 4, 'ΕΝΝΕΑ'),
  grid.word(5, 0, 'ΔΕΚΑ'),
  grid.word(5, 4, 'ΕΝΤΕΚΑ'),
];

const minutes: Cells[][] = [
  [],
  [w.kai, w.pente],
  [w.kai, w.deka],
  [w.kai, w.tetarto],
  [w.kai, w.eikosi],
  [w.kai, w.eikosipente],
  [w.kai, w.misi],
  [w.para, w.eikosipente],
  [w.para, w.eikosi],
  [w.para, w.tetarto],
  [w.para, w.deka],
  [w.para, w.pente],
];

export const el: Locale = {
  id: 'el',
  name: 'Griechisch',
  lang: 'el-GR',
  grid,
  words: [...Object.values(w), ...hours],
  frame(now, { intro }) {
    const step = Math.floor(now.getMinutes() / 5);
    const h = (step >= 7 ? now.getHours() + 1 : now.getHours()) % 12;
    const intros = intro ? [w.einai] : [];
    return { words: [...intros, hours[h]!, ...minutes[step]!], dots: now.getMinutes() % 5 };
  },
};
