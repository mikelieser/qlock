import { createGrid, type Cells } from '../core/grid';
import type { Locale } from '../core/locale';

const grid = createGrid([
  'ESONELASUNA',
  'DOSITRESOAM',
  'CUATROCINCO',
  'SEISASIETEN',
  'OCHONUEVEPM',
  'LADIEZSONCE',
  'DOCELYMENOS',
  'OVEINTEDIEZ',
  'VEINTICINCO',
  'MEDIACUARTO',
]);

const w = {
  es: grid.word(0, 0, 'ES'),
  son: grid.word(0, 1, 'SON'),
  la: grid.word(0, 5, 'LA'),
  las: grid.word(0, 5, 'LAS'),
  y: grid.word(6, 5, 'Y'),
  menos: grid.word(6, 6, 'MENOS'),
  veinte: grid.word(7, 1, 'VEINTE'),
  diez: grid.word(7, 7, 'DIEZ'),
  veinticinco: grid.word(8, 0, 'VEINTICINCO'),
  cinco: grid.word(8, 6, 'CINCO'),
  media: grid.word(9, 0, 'MEDIA'),
  cuarto: grid.word(9, 5, 'CUARTO'),
};

// Index = hour % 12, so index 0 is DOCE.
const hours: Cells[] = [
  grid.word(6, 0, 'DOCE'),
  grid.word(0, 8, 'UNA'),
  grid.word(1, 0, 'DOS'),
  grid.word(1, 4, 'TRES'),
  grid.word(2, 0, 'CUATRO'),
  grid.word(2, 6, 'CINCO'),
  grid.word(3, 0, 'SEIS'),
  grid.word(3, 5, 'SIETE'),
  grid.word(4, 0, 'OCHO'),
  grid.word(4, 4, 'NUEVE'),
  grid.word(5, 2, 'DIEZ'),
  grid.word(5, 7, 'ONCE'),
];

const minutes: Cells[][] = [
  [],
  [w.y, w.cinco],
  [w.y, w.diez],
  [w.y, w.cuarto],
  [w.y, w.veinte],
  [w.y, w.veinticinco],
  [w.y, w.media],
  [w.menos, w.veinticinco],
  [w.menos, w.veinte],
  [w.menos, w.cuarto],
  [w.menos, w.diez],
  [w.menos, w.cinco],
];

export const es: Locale = {
  id: 'es',
  name: 'Spanisch',
  lang: 'es-ES',
  grid,
  words: [...Object.values(w), ...hours],
  frame(now, { intro }) {
    const step = Math.floor(now.getMinutes() / 5);
    const h = (step >= 7 ? now.getHours() + 1 : now.getHours()) % 12;
    // "Es la una", but "son las dos".
    const intros = intro ? (h === 1 ? [w.es, w.la] : [w.son, w.las]) : [];
    return { words: [...intros, hours[h]!, ...minutes[step]!], dots: now.getMinutes() % 5 };
  },
};
