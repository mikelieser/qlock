import { createGrid, type Cells } from '../core/grid';
import type { Locale } from '../core/locale';

const grid = createGrid([
  'ITLISASTIME',
  'ACQUARTEROC',
  'TWENTYFIVEX',
  'HALFBTENFTO',
  'PASTERUNINE',
  'ONESIXTHREE',
  'FOURFIVETWO',
  'EIGHTELEVEN',
  'SEVENTWELVE',
  'TENSEOCLOCK',
]);

const w = {
  it: grid.word(0, 0, 'IT'),
  is: grid.word(0, 3, 'IS'),
  a: grid.word(1, 0, 'A'),
  quarter: grid.word(1, 2, 'QUARTER'),
  twenty: grid.word(2, 0, 'TWENTY'),
  five: grid.word(2, 6, 'FIVE'),
  half: grid.word(3, 0, 'HALF'),
  ten: grid.word(3, 5, 'TEN'),
  to: grid.word(3, 9, 'TO'),
  past: grid.word(4, 0, 'PAST'),
  oclock: grid.word(9, 5, 'OCLOCK'),
};

// Index = hour % 12, so index 0 is TWELVE.
const hours: Cells[] = [
  grid.word(8, 5, 'TWELVE'),
  grid.word(5, 0, 'ONE'),
  grid.word(6, 8, 'TWO'),
  grid.word(5, 6, 'THREE'),
  grid.word(6, 0, 'FOUR'),
  grid.word(6, 4, 'FIVE'),
  grid.word(5, 3, 'SIX'),
  grid.word(8, 0, 'SEVEN'),
  grid.word(7, 0, 'EIGHT'),
  grid.word(4, 7, 'NINE'),
  grid.word(9, 0, 'TEN'),
  grid.word(7, 5, 'ELEVEN'),
];

export const en: Locale = {
  id: 'en',
  name: 'Englisch',
  lang: 'en-GB',
  grid,
  words: [...Object.values(w), ...hours],
  frame(now, { intro }) {
    const h = now.getHours() % 12;
    const m = now.getMinutes();
    const hour = hours[h]!;
    const next = hours[(h + 1) % 12]!;

    const phrases: Cells[][] = [
      [hour, w.oclock],
      [w.five, w.past, hour],
      [w.ten, w.past, hour],
      [w.a, w.quarter, w.past, hour],
      [w.twenty, w.past, hour],
      [w.twenty, w.five, w.past, hour],
      [w.half, w.past, hour],
      [w.twenty, w.five, w.to, next],
      [w.twenty, w.to, next],
      [w.a, w.quarter, w.to, next],
      [w.ten, w.to, next],
      [w.five, w.to, next],
    ];

    const intros = intro ? [w.it, w.is] : [];
    return { words: [...intros, ...phrases[Math.floor(m / 5)]!], dots: m % 5 };
  },
};
