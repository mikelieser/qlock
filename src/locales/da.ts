import { minutesFirst } from '../core/clock';
import { createGrid } from '../core/grid';

const grid = createGrid([
  'KLOKKENVERS',
  'FEMTYVETIKA',
  'KVARTOVERXI',
  'HALVETTOTRE',
  'FIREFEMSEKS',
  'SYVOTTENITI',
  'ELLEVETOLVA',
  'QHYGGEMKLSB',
  'ZÆØÅRDPMNTU',
  'GLØBVRÆKPOJ',
]);

const fem = grid.word(1, 0, 'FEM');
const tyve = grid.word(1, 3, 'TYVE');
const ti = grid.word(1, 7, 'TI');
const kvart = grid.word(2, 0, 'KVART');
const over = grid.word(2, 5, 'OVER');
const i = grid.word(2, 10, 'I');
const halv = grid.word(3, 0, 'HALV');

// Like Swedish: "fem i halv fire" is 3:25. Spoken Danish often drops "minutter", which keeps the grid small.
export const da = minutesFirst({
  id: 'da',
  name: 'Dänisch',
  lang: 'da-DK',
  grid,
  intro: [grid.word(0, 0, 'KLOKKEN'), grid.word(0, 8, 'ER')],
  hours: [
    grid.word(6, 6, 'TOLV'),
    grid.word(3, 4, 'ET'),
    grid.word(3, 6, 'TO'),
    grid.word(3, 8, 'TRE'),
    grid.word(4, 0, 'FIRE'),
    grid.word(4, 4, 'FEM'),
    grid.word(4, 7, 'SEKS'),
    grid.word(5, 0, 'SYV'),
    grid.word(5, 3, 'OTTE'),
    grid.word(5, 7, 'NI'),
    grid.word(5, 9, 'TI'),
    grid.word(6, 0, 'ELLEVE'),
  ],
  minutes: [
    [],
    [fem, over],
    [ti, over],
    [kvart, over],
    [tyve, over],
    [fem, i, halv],
    [halv],
    [fem, over, halv],
    [tyve, i],
    [kvart, i],
    [ti, i],
    [fem, i],
  ],
  nextHourFrom: 5,
});
