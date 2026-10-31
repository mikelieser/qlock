import { minutesFirst } from '../core/clock';
import { createGrid } from '../core/grid';

const grid = createGrid([
  'HETKISAVIJF',
  'TIENZKWARTS',
  'VOOROVERMEK',
  'HALFXTWEEEN',
  'DRIEVIERZES',
  'VIJFZEVENOM',
  'ACHTNEGENKL',
  'TIENRELFPQS',
  'TWAALFSBUUR',
  'BQMEVROSTKA',
]);

const vijf = grid.word(0, 7, 'VIJF');
const tien = grid.word(1, 0, 'TIEN');
const kwart = grid.word(1, 5, 'KWART');
const voor = grid.word(2, 0, 'VOOR');
const over = grid.word(2, 4, 'OVER');
const half = grid.word(3, 0, 'HALF');

// Dutch counts towards the half hour from :20 on: "tien voor half vier" is 3:20.
export const nl = minutesFirst({
  id: 'nl',
  name: 'Niederländisch',
  lang: 'nl-NL',
  grid,
  intro: [grid.word(0, 0, 'HET'), grid.word(0, 4, 'IS')],
  hours: [
    grid.word(8, 0, 'TWAALF'),
    grid.word(3, 8, 'EEN'),
    grid.word(3, 5, 'TWEE'),
    grid.word(4, 0, 'DRIE'),
    grid.word(4, 4, 'VIER'),
    grid.word(5, 0, 'VIJF'),
    grid.word(4, 8, 'ZES'),
    grid.word(5, 4, 'ZEVEN'),
    grid.word(6, 0, 'ACHT'),
    grid.word(6, 4, 'NEGEN'),
    grid.word(7, 0, 'TIEN'),
    grid.word(7, 5, 'ELF'),
  ],
  minutes: [
    [],
    [vijf, over],
    [tien, over],
    [kwart, over],
    [tien, voor, half],
    [vijf, voor, half],
    [half],
    [vijf, over, half],
    [tien, over, half],
    [kwart, voor],
    [tien, voor],
    [vijf, voor],
  ],
  nextHourFrom: 4,
  onTheHour: [grid.word(8, 8, 'UUR')],
});
