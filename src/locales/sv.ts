import { createGrid, type Cells } from '../core/grid';
import type { Locale } from '../core/locale';

const grid = createGrid([
  'KLOCKANTÄRK',
  'FEMTJUGOTIO',
  'KVARTIÖVERS',
  'HALVETTVÅKE',
  'TREMFYRAFEM',
  'SEXASJUÅTTA',
  'NIOTTIOELVA',
  'TOLVRSGYPNE',
  'QÅDÖLPMYRBZ',
  'JUHVÄGNOSKL',
]);

const w = {
  klockan: grid.word(0, 0, 'KLOCKAN'),
  ar: grid.word(0, 8, 'ÄR'),
  fem: grid.word(1, 0, 'FEM'),
  tjugo: grid.word(1, 3, 'TJUGO'),
  tio: grid.word(1, 8, 'TIO'),
  kvart: grid.word(2, 0, 'KVART'),
  i: grid.word(2, 5, 'I'),
  over: grid.word(2, 6, 'ÖVER'),
  halv: grid.word(3, 0, 'HALV'),
};

// Index = hour % 12, so index 0 is TOLV.
const hours: Cells[] = [
  grid.word(7, 0, 'TOLV'),
  grid.word(3, 4, 'ETT'),
  grid.word(3, 6, 'TVÅ'),
  grid.word(4, 0, 'TRE'),
  grid.word(4, 4, 'FYRA'),
  grid.word(4, 8, 'FEM'),
  grid.word(5, 0, 'SEX'),
  grid.word(5, 4, 'SJU'),
  grid.word(5, 7, 'ÅTTA'),
  grid.word(6, 0, 'NIO'),
  grid.word(6, 4, 'TIO'),
  grid.word(6, 7, 'ELVA'),
];

// Swedish counts half hours towards the next hour: "fem i halv fyra" is 3:25.
const minutes: Cells[][] = [
  [],
  [w.fem, w.over],
  [w.tio, w.over],
  [w.kvart, w.over],
  [w.tjugo, w.over],
  [w.fem, w.i, w.halv],
  [w.halv],
  [w.fem, w.over, w.halv],
  [w.tjugo, w.i],
  [w.kvart, w.i],
  [w.tio, w.i],
  [w.fem, w.i],
];

export const sv: Locale = {
  id: 'sv',
  name: 'Schwedisch',
  lang: 'sv-SE',
  grid,
  words: [...Object.values(w), ...hours],
  frame(now, { intro }) {
    const step = Math.floor(now.getMinutes() / 5);
    const h = (step >= 5 ? now.getHours() + 1 : now.getHours()) % 12;
    const intros = intro ? [w.klockan, w.ar] : [];
    return { words: [...intros, ...minutes[step]!, hours[h]!], dots: now.getMinutes() % 5 };
  },
};
