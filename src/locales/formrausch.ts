import { allGermanWords, germanTime, type GermanWords } from '../core/german';
import { createGrid, type Cells } from '../core/grid';
import type { Locale } from '../core/locale';

const grid = createGrid([
  'ESRISTSFÜNF',
  'ZEHNZWANZIG',
  'DREIVIERTEL',
  'VøRFøRMNACH',
  'HALBRELFÜNF',
  'EINSAATZWEI',
  'DREIUSCVIER',
  'SECHSΣHACHT',
  'SIEBENZWøLF',
  'ZEHNEUNDUHR',
]);

const w: GermanWords = {
  es: grid.word(0, 0, 'ES'),
  ist: grid.word(0, 3, 'IST'),
  fuenf: grid.word(0, 7, 'FÜNF'),
  zehn: grid.word(1, 0, 'ZEHN'),
  zwanzig: grid.word(1, 4, 'ZWANZIG'),
  viertel: grid.word(2, 4, 'VIERTEL'),
  vor: grid.word(3, 0, 'VøR'),
  nach: grid.word(3, 7, 'NACH'),
  halb: grid.word(4, 0, 'HALB'),
  uhr: grid.word(9, 8, 'UHR'),
  ein: grid.word(5, 0, 'EIN'),
  hours: [
    grid.word(8, 6, 'ZWøLF'),
    grid.word(5, 0, 'EINS'),
    grid.word(5, 7, 'ZWEI'),
    grid.word(6, 0, 'DREI'),
    grid.word(6, 7, 'VIER'),
    grid.word(4, 7, 'FÜNF'),
    grid.word(7, 0, 'SECHS'),
    grid.word(8, 0, 'SIEBEN'),
    grid.word(7, 7, 'ACHT'),
    grid.word(9, 3, 'NEUN'),
    grid.word(9, 0, 'ZEHN'),
    grid.word(4, 5, 'ELF'),
  ],
};

// Free-form pictures, written as 1-based cell numbers like in the original grid sketch.
const cells = (...oneBased: number[]): Cells => oneBased.map((n) => n - 1);
const feierabend = cells(11, 25, 26, 29, 30, 61, 92, 93, 106, 107);
const formrausch = cells(37, 38, 39, 40, 49, 60, 71, 72, 73, 84);
const happa = cells(
  27, 28, 29, 38, 39, 40, 49, 50, 51, 60, 61, 62, 71, 72, 73, 82, 83, 84, 6, 105, 25, 47, 69, 86, 64, 42,
);

export const formrauschLocale: Locale = {
  id: 'formrausch',
  name: 'Formrausch',
  lang: 'de-DE',
  grid,
  words: allGermanWords(w),
  frame(now, { intro }) {
    const h = now.getHours();
    const m = now.getMinutes();

    if (h === 11 && m >= 15 && m <= 45 && m % 2 === 1) {
      return { words: [happa], dots: 4, title: 'FRÜHSTÜCKSZEIT ;)' };
    }
    if (h === 18) {
      const words = m % 2 === 0 ? [w.es, w.ist, feierabend] : [formrausch];
      return { words, dots: 4, title: 'FEIERABEND ;)' };
    }
    return { words: germanTime(w, now, intro), dots: m % 5 };
  },
};
