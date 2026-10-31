import { allGermanWords, germanTime, type GermanWords } from '../core/german';
import { createGrid } from '../core/grid';
import type { Locale } from '../core/locale';

const grid = createGrid([
  'ESKISTAFÜNF',
  'ZEHNZWANZIG',
  'DREIVIERTEL',
  'VORFUNKNACH',
  'HALBAELFÜNF',
  'EINSXÄMZWEI',
  'DREIAUJVIER',
  'SECHSNLACHT',
  'SIEBENZWÖLF',
  'ZEHNEUNKUHR',
]);

const w: GermanWords = {
  es: grid.word(0, 0, 'ES'),
  ist: grid.word(0, 3, 'IST'),
  fuenf: grid.word(0, 7, 'FÜNF'),
  zehn: grid.word(1, 0, 'ZEHN'),
  zwanzig: grid.word(1, 4, 'ZWANZIG'),
  viertel: grid.word(2, 4, 'VIERTEL'),
  vor: grid.word(3, 0, 'VOR'),
  nach: grid.word(3, 7, 'NACH'),
  halb: grid.word(4, 0, 'HALB'),
  uhr: grid.word(9, 8, 'UHR'),
  ein: grid.word(5, 0, 'EIN'),
  hours: [
    grid.word(8, 6, 'ZWÖLF'),
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

export const de: Locale = {
  id: 'de',
  name: 'Deutsch',
  lang: 'de-DE',
  grid,
  words: allGermanWords(w),
  frame: (now, { intro }) => ({ words: germanTime(w, now, intro), dots: now.getMinutes() % 5 }),
};
