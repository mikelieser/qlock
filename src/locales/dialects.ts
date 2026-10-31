import { minutesFirst } from '../core/clock';
import { createGrid, type Cells, type Grid } from '../core/grid';

/*
 * German dialects. Swabian, Bavarian and Berlin count quarters towards the next hour
 * ("viertel vier" = 3:15, "dreiviertel vier" = 3:45); Swiss German, Hamburg Platt and Hessian
 * say "viertel ab/na/nach". Grids are our own; the bottom row hides a regional word as filler.
 */

interface QuarterWords {
  fuenf: Cells;
  zehn: Cells;
  nach: Cells;
  vor: Cells;
  halb: Cells;
  viertel: Cells;
  dreiviertel: Cells;
}

/** Minute phrases for the quarter-towards-next-hour dialects; everything from :15 on names the next hour. */
function quarterMinutes({ fuenf, zehn, nach, vor, halb, viertel, dreiviertel }: QuarterWords): Cells[][] {
  return [
    [],
    [fuenf, nach],
    [zehn, nach],
    [viertel],
    [zehn, vor, halb],
    [fuenf, vor, halb],
    [halb],
    [fuenf, nach, halb],
    [zehn, nach, halb],
    [dreiviertel],
    [zehn, vor],
    [fuenf, vor],
  ];
}

/** Shorthand for grid.word on one grid. */
const on = (grid: Grid) => (row: number, col: number, text: string) => grid.word(row, col, text);

const swaGrid = createGrid([
  'ESKISCHFÜNF',
  'ZEHNNACHVOR',
  'DREIVIERTEL',
  'HALBMOINSAB',
  'ZWOIRDREIUS',
  'VIERFÜNFELF',
  'SECHSSIEBAX',
  'ACHTKNEUNOS',
  'ZEHNZWÖLFLE',
  'SCHAFFAXBIZ',
]);

export const swabian = minutesFirst({
  id: 'swa',
  name: 'Schwäbisch',
  lang: 'de-DE',
  grid: swaGrid,
  intro: [swaGrid.word(0, 0, 'ES'), swaGrid.word(0, 3, 'ISCH')],
  hours: [
    swaGrid.word(8, 4, 'ZWÖLF'),
    swaGrid.word(3, 5, 'OINS'),
    swaGrid.word(4, 0, 'ZWOI'),
    swaGrid.word(4, 5, 'DREI'),
    swaGrid.word(5, 0, 'VIER'),
    swaGrid.word(5, 4, 'FÜNF'),
    swaGrid.word(6, 0, 'SECHS'),
    swaGrid.word(6, 5, 'SIEBA'),
    swaGrid.word(7, 0, 'ACHT'),
    swaGrid.word(7, 5, 'NEUN'),
    swaGrid.word(8, 0, 'ZEHN'),
    swaGrid.word(5, 8, 'ELF'),
  ],
  minutes: quarterMinutes({
    fuenf: swaGrid.word(0, 7, 'FÜNF'),
    zehn: swaGrid.word(1, 0, 'ZEHN'),
    nach: swaGrid.word(1, 4, 'NACH'),
    vor: swaGrid.word(1, 8, 'VOR'),
    halb: swaGrid.word(3, 0, 'HALB'),
    viertel: swaGrid.word(2, 4, 'VIERTEL'),
    dreiviertel: swaGrid.word(2, 0, 'DREIVIERTEL'),
  }),
  nextHourFrom: 3,
});

const barGrid = createGrid([
  'ESXISFÜMFKA',
  'ZEHNNACHVOR',
  'DREIVIERTLB',
  'HOIBEOANSXZ',
  'ZWOAKDREIRS',
  'VIERFÜMFELF',
  'SECHSSIEBNA',
  'ACHTKNEUNOS',
  'ZEHNZWÄIFLE',
  'OZAPFTISBQM',
]);

export const bavarian = minutesFirst({
  id: 'bar',
  name: 'Bayerisch',
  lang: 'de-DE',
  grid: barGrid,
  intro: [barGrid.word(0, 0, 'ES'), barGrid.word(0, 3, 'IS')],
  hours: [
    barGrid.word(8, 4, 'ZWÄIF'),
    barGrid.word(3, 5, 'OANS'),
    barGrid.word(4, 0, 'ZWOA'),
    barGrid.word(4, 5, 'DREI'),
    barGrid.word(5, 0, 'VIER'),
    barGrid.word(5, 4, 'FÜMF'),
    barGrid.word(6, 0, 'SECHS'),
    barGrid.word(6, 5, 'SIEBN'),
    barGrid.word(7, 0, 'ACHT'),
    barGrid.word(7, 5, 'NEUN'),
    barGrid.word(8, 0, 'ZEHN'),
    barGrid.word(5, 8, 'ELF'),
  ],
  minutes: quarterMinutes({
    fuenf: barGrid.word(0, 5, 'FÜMF'),
    zehn: barGrid.word(1, 0, 'ZEHN'),
    nach: barGrid.word(1, 4, 'NACH'),
    vor: barGrid.word(1, 8, 'VOR'),
    halb: barGrid.word(3, 0, 'HOIBE'),
    viertel: barGrid.word(2, 4, 'VIERTL'),
    dreiviertel: barGrid.word(2, 0, 'DREIVIERTL'),
  }),
  nextHourFrom: 3,
});

const berGrid = createGrid([
  'ETKISAFÜNFR',
  'ZEHNNACHVOR',
  'DREIVIERTEL',
  'HALBXEENSPM',
  'ZWEEKDREIRS',
  'VIERFÜNFELF',
  'SECHSSIEBEN',
  'ACHTKNEUNOS',
  'ZEHNZWÖLFWA',
  'CURRYWURSTX',
]);

export const berlin = minutesFirst({
  id: 'ber',
  name: 'Berlinerisch',
  lang: 'de-DE',
  grid: berGrid,
  intro: [berGrid.word(0, 0, 'ET'), berGrid.word(0, 3, 'IS')],
  hours: [
    berGrid.word(8, 4, 'ZWÖLF'),
    berGrid.word(3, 5, 'EENS'),
    berGrid.word(4, 0, 'ZWEE'),
    berGrid.word(4, 5, 'DREI'),
    berGrid.word(5, 0, 'VIER'),
    berGrid.word(5, 4, 'FÜNF'),
    berGrid.word(6, 0, 'SECHS'),
    berGrid.word(6, 5, 'SIEBEN'),
    berGrid.word(7, 0, 'ACHT'),
    berGrid.word(7, 5, 'NEUN'),
    berGrid.word(8, 0, 'ZEHN'),
    berGrid.word(5, 8, 'ELF'),
  ],
  minutes: quarterMinutes({
    fuenf: berGrid.word(0, 6, 'FÜNF'),
    zehn: berGrid.word(1, 0, 'ZEHN'),
    nach: berGrid.word(1, 4, 'NACH'),
    vor: berGrid.word(1, 8, 'VOR'),
    halb: berGrid.word(3, 0, 'HALB'),
    viertel: berGrid.word(2, 4, 'VIERTEL'),
    dreiviertel: berGrid.word(2, 0, 'DREIVIERTEL'),
  }),
  nextHourFrom: 3,
  outro: [berGrid.word(8, 9, 'WA')],
});

const gswGrid = createGrid([
  'ESKISCHAFÜF',
  'VIERTELNZÄH',
  'ZWÄNZGABVOR',
  'HALBIKEISXZ',
  'ZWEIMDRÜÜPS',
  'VIERIFÜFIRT',
  'SÄCHSISIBNI',
  'ACHTINÜNIBK',
  'ZÄNIXELFIOU',
  'ZWÖLFIMERCI',
]);

const gsw = on(gswGrid);
const gswFuef = gsw(0, 8, 'FÜF');
const gswZaeh = gsw(1, 8, 'ZÄH');
const gswViertel = gsw(1, 0, 'VIERTEL');
const gswZwaenzg = gsw(2, 0, 'ZWÄNZG');
const gswAb = gsw(2, 6, 'AB');
const gswVor = gsw(2, 8, 'VOR');
const gswHalbi = gsw(3, 0, 'HALBI');

export const swissGerman = minutesFirst({
  id: 'gsw',
  name: 'Schweizerdeutsch',
  lang: 'de-CH',
  grid: gswGrid,
  intro: [gsw(0, 0, 'ES'), gsw(0, 3, 'ISCH')],
  hours: [
    gsw(9, 0, 'ZWÖLFI'),
    gsw(3, 6, 'EIS'),
    gsw(4, 0, 'ZWEI'),
    gsw(4, 5, 'DRÜÜ'),
    gsw(5, 0, 'VIERI'),
    gsw(5, 5, 'FÜFI'),
    gsw(6, 0, 'SÄCHSI'),
    gsw(6, 6, 'SIBNI'),
    gsw(7, 0, 'ACHTI'),
    gsw(7, 5, 'NÜNI'),
    gsw(8, 0, 'ZÄNI'),
    gsw(8, 5, 'ELFI'),
  ],
  minutes: [
    [],
    [gswFuef, gswAb],
    [gswZaeh, gswAb],
    [gswViertel, gswAb],
    [gswZwaenzg, gswAb],
    [gswFuef, gswVor, gswHalbi],
    [gswHalbi],
    [gswFuef, gswAb, gswHalbi],
    [gswZwaenzg, gswVor],
    [gswViertel, gswVor],
    [gswZaeh, gswVor],
    [gswFuef, gswVor],
  ],
  nextHourFrom: 5,
});

const ndsGrid = createGrid([
  'KLOCKSISFUJ',
  'FIEFTWINTIG',
  'VEERDELMOIN',
  'TEIHNNAVÖRX',
  'HALFEENTWEE',
  'DREEKVEERPS',
  'FIEFRSÖSSUA',
  'SÖBENACHTMK',
  'NEGENTEIHNX',
  'ÖLVENTWÖLFA',
]);

const nds = on(ndsGrid);
const ndsFief = nds(1, 0, 'FIEF');
const ndsTwintig = nds(1, 4, 'TWINTIG');
const ndsVeerdel = nds(2, 0, 'VEERDEL');
const ndsTeihn = nds(3, 0, 'TEIHN');
const ndsNa = nds(3, 5, 'NA');
const ndsVoer = nds(3, 7, 'VÖR');
const ndsHalf = nds(4, 0, 'HALF');

export const hamburgPlatt = minutesFirst({
  id: 'nds',
  name: 'Hamburger Platt',
  lang: 'de-DE',
  grid: ndsGrid,
  intro: [nds(0, 0, 'KLOCK'), nds(0, 6, 'IS')],
  hours: [
    nds(9, 5, 'TWÖLF'),
    nds(4, 4, 'EEN'),
    nds(4, 7, 'TWEE'),
    nds(5, 0, 'DREE'),
    nds(5, 5, 'VEER'),
    nds(6, 0, 'FIEF'),
    nds(6, 5, 'SÖSS'),
    nds(7, 0, 'SÖBEN'),
    nds(7, 5, 'ACHT'),
    nds(8, 0, 'NEGEN'),
    nds(8, 5, 'TEIHN'),
    nds(9, 0, 'ÖLVEN'),
  ],
  minutes: [
    [],
    [ndsFief, ndsNa],
    [ndsTeihn, ndsNa],
    [ndsVeerdel, ndsNa],
    [ndsTwintig, ndsNa],
    [ndsFief, ndsVoer, ndsHalf],
    [ndsHalf],
    [ndsFief, ndsNa, ndsHalf],
    [ndsTwintig, ndsVoer],
    [ndsVeerdel, ndsVoer],
    [ndsTeihn, ndsVoer],
    [ndsFief, ndsVoer],
  ],
  nextHourFrom: 5,
});

const hesGrid = createGrid([
  'ESKISAFINFX',
  'ZEHVIERTELB',
  'ZWANZISCHKS',
  'NACHVORHALB',
  'AANSKZWAAPS',
  'DREIBVIERMU',
  'FINFSECHSXA',
  'SIEWEACHTKL',
  'NEUNZEHNELF',
  'ZWELFAGELLO',
]);

const hes = on(hesGrid);
const hesFinf = hes(0, 6, 'FINF');
const hesZeh = hes(1, 0, 'ZEH');
const hesViertel = hes(1, 3, 'VIERTEL');
const hesZwanzisch = hes(2, 0, 'ZWANZISCH');
const hesNach = hes(3, 0, 'NACH');
const hesVor = hes(3, 4, 'VOR');
const hesHalb = hes(3, 7, 'HALB');

export const hessian = minutesFirst({
  id: 'hes',
  name: 'Hessisch',
  lang: 'de-DE',
  grid: hesGrid,
  intro: [hes(0, 0, 'ES'), hes(0, 3, 'IS')],
  hours: [
    hes(9, 0, 'ZWELF'),
    hes(4, 0, 'AANS'),
    hes(4, 5, 'ZWAA'),
    hes(5, 0, 'DREI'),
    hes(5, 5, 'VIER'),
    hes(6, 0, 'FINF'),
    hes(6, 4, 'SECHS'),
    hes(7, 0, 'SIEWE'),
    hes(7, 5, 'ACHT'),
    hes(8, 0, 'NEUN'),
    hes(8, 4, 'ZEHN'),
    hes(8, 8, 'ELF'),
  ],
  minutes: [
    [],
    [hesFinf, hesNach],
    [hesZeh, hesNach],
    [hesViertel, hesNach],
    [hesZwanzisch, hesNach],
    [hesFinf, hesVor, hesHalb],
    [hesHalb],
    [hesFinf, hesNach, hesHalb],
    [hesZwanzisch, hesVor],
    [hesViertel, hesVor],
    [hesZeh, hesVor],
    [hesFinf, hesVor],
  ],
  nextHourFrom: 5,
  outro: [hes(9, 6, 'GELL')],
});

const kshGrid = createGrid([
  'ETMESFÜNNEF',
  'ZEHNVEEDELS',
  'ZWANZICHNOH',
  'VÖRHALVEINS',
  'ZWEIXDREIBA',
  'VEERFÜNNEFX',
  'SECHSSIBBEP',
  'AACHNÜNGRTZ',
  'ZEHNELLEFKO',
  'ZWÖLFALAAFS',
]);

const ksh = on(kshGrid);
const kshFuennef = ksh(0, 5, 'FÜNNEF');
const kshZehn = ksh(1, 0, 'ZEHN');
const kshVeedel = ksh(1, 4, 'VEEDEL');
const kshZwanzich = ksh(2, 0, 'ZWANZICH');
const kshNoh = ksh(2, 8, 'NOH');
const kshVoer = ksh(3, 0, 'VÖR');
const kshHalv = ksh(3, 3, 'HALV');

export const cologne = minutesFirst({
  id: 'ksh',
  name: 'Kölsch',
  lang: 'de-DE',
  grid: kshGrid,
  intro: [ksh(0, 0, 'ET'), ksh(0, 3, 'ES')],
  hours: [
    ksh(9, 0, 'ZWÖLF'),
    ksh(3, 7, 'EINS'),
    ksh(4, 0, 'ZWEI'),
    ksh(4, 5, 'DREI'),
    ksh(5, 0, 'VEER'),
    ksh(5, 4, 'FÜNNEF'),
    ksh(6, 0, 'SECHS'),
    ksh(6, 5, 'SIBBE'),
    ksh(7, 0, 'AACH'),
    ksh(7, 4, 'NÜNG'),
    ksh(8, 0, 'ZEHN'),
    ksh(8, 4, 'ELLEF'),
  ],
  minutes: [
    [],
    [kshFuennef, kshNoh],
    [kshZehn, kshNoh],
    [kshVeedel, kshNoh],
    [kshZwanzich, kshNoh],
    [kshFuennef, kshVoer, kshHalv],
    [kshHalv],
    [kshFuennef, kshNoh, kshHalv],
    [kshZwanzich, kshVoer],
    [kshVeedel, kshVoer],
    [kshZehn, kshVoer],
    [kshFuennef, kshVoer],
  ],
  nextHourFrom: 5,
});
