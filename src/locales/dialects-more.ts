import { minutesFirst } from '../core/clock';
import type { Cells } from '../core/grid';
import { layoutGrid, type LayoutRow } from '../core/layout';
import type { Locale } from '../core/locale';

/*
 * More German dialects, laid out with layoutGrid. Two families:
 * - "viertel drei" (quarter towards the next hour): Saxon, Viennese, Franconian, Baden
 * - "viertel nach/vor" with "zwanzig": Palatinate, Saarland, Ruhr, East Frisian, Koblenz
 * Forms are written the way they are commonly spelled; regional variants exist.
 */

const HOURS = ['h12', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'h7', 'h8', 'h9', 'h10', 'h11'];

interface DialectSpec {
  id: string;
  name: string;
  lang?: string;
  rows: LayoutRow[];
  filler: string;
  intro: string[];
  outro?: string[];
}

function build(spec: DialectSpec, minutes: (w: Record<string, Cells>) => Cells[][], nextHourFrom: number): Locale {
  const { grid, w } = layoutGrid(spec.rows, spec.filler);
  return minutesFirst({
    id: spec.id,
    name: spec.name,
    lang: spec.lang ?? 'de-DE',
    grid,
    intro: spec.intro.map((k) => w[k]!),
    outro: spec.outro?.map((k) => w[k]!),
    hours: HOURS.map((k) => w[k]!),
    minutes: minutes(w),
    nextHourFrom,
  });
}

/** Keys: fuenf, zehn, nach, vor, halb, dv (dreiviertel, with viertel as its tail). */
const quarter = (spec: DialectSpec, viertelLength: number) =>
  build(
    spec,
    (w) => {
      const viertel = w.dv!.slice(-viertelLength);
      const { fuenf, zehn, nach, vor, halb, dv } = w as Record<string, Cells>;
      return [
        [],
        [fuenf!, nach!],
        [zehn!, nach!],
        [viertel],
        [zehn!, vor!, halb!],
        [fuenf!, vor!, halb!],
        [halb!],
        [fuenf!, nach!, halb!],
        [zehn!, nach!, halb!],
        [dv!],
        [zehn!, vor!],
        [fuenf!, vor!],
      ];
    },
    3,
  );

/** Keys: fuenf, zehn, viertel, zwanzig, nach, vor, halb. */
const nachVor = (spec: DialectSpec) =>
  build(
    spec,
    (w) => {
      const { fuenf, zehn, viertel, zwanzig, nach, vor, halb } = w as Record<string, Cells>;
      return [
        [],
        [fuenf!, nach!],
        [zehn!, nach!],
        [viertel!, nach!],
        [zwanzig!, nach!],
        [fuenf!, vor!, halb!],
        [halb!],
        [fuenf!, nach!, halb!],
        [zwanzig!, vor!],
        [viertel!, vor!],
        [zehn!, vor!],
        [fuenf!, vor!],
      ];
    },
    5,
  );

export const saxon = quarter(
  {
    id: 'sxu',
    name: 'Sächsisch',
    filler: 'KBXUPÄLTMRÖ',
    intro: ['es', 'is'],
    rows: [
      [['es', 'ES'], ['is', 'IS'], ['fuenf', 'FÜMF']],
      [['zehn', 'ZEHN'], ['nach', 'NACH'], ['vor', 'VOR']],
      [['dv', 'DREIVIERDL']],
      [['halb', 'HALB'], ['h1', 'EENS']],
      [['h2', 'ZWEE'], ['h3', 'DREIE']],
      [['h4', 'VIERE'], ['h5', 'FÜMFE']],
      [['h6', 'SECHSE'], ['h11', 'ELFE']],
      [['h7', 'SIEBNE'], ['h8', 'ACHTE']],
      [['h9', 'NEINE'], ['h10', 'ZEHNE']],
      [['h12', 'ZWÖLFE'], ['x', 'BEMME']],
    ],
  },
  6,
);

export const viennese = quarter(
  {
    id: 'wien',
    name: 'Wienerisch',
    lang: 'de-AT',
    filler: 'QÜKTRBPXJMZ',
    intro: ['es', 'is'],
    rows: [
      [['es', 'ES'], ['is', 'IS'], ['fuenf', 'FÜNF']],
      [['zehn', 'ZEHN'], ['nach', 'NACH'], ['vor', 'VOR']],
      [['dv', 'DREIVIERTL']],
      [['halb', 'HOIB'], ['h1', 'ANS'], ['h2', 'ZWA']],
      [['h3', 'DREI'], ['h4', 'VIER']],
      [['h5', 'FÜNF'], ['h6', 'SECHS']],
      [['h7', 'SIEBM'], ['h8', 'ACHT']],
      [['h9', 'NEIN'], ['h10', 'ZEHN'], ['h11', 'ÖF']],
      [['h12', 'ZWÖF'], ['x', 'SCHMÄH']],
      [['y', 'LEIWAND']],
    ],
  },
  6,
);

export const franconian = quarter(
  {
    id: 'frk',
    name: 'Fränkisch',
    filler: 'PXGTQÖRBMKL',
    intro: ['es', 'is'],
    rows: [
      [['es', 'ES'], ['is', 'IS'], ['fuenf', 'FÜMF']],
      [['zehn', 'ZEH'], ['nach', 'NACH'], ['vor', 'VOR']],
      [['dv', 'DREIVIERDL']],
      [['halb', 'HALB'], ['h1', 'AANS']],
      [['h2', 'ZWAA'], ['h3', 'DREI']],
      [['h4', 'VIER'], ['h5', 'FÜMF']],
      [['h6', 'SECHS'], ['h7', 'SIEM']],
      [['h8', 'ACHT'], ['h9', 'NEUN']],
      [['h10', 'ZEH'], ['h11', 'ELF'], ['h12', 'ZWÖLF']],
      [['x', 'BRATWURST']],
    ],
  },
  6,
);

export const badish = quarter(
  {
    id: 'bad',
    name: 'Badisch',
    filler: 'XQÖKTPRBMJL',
    intro: ['es', 'isch'],
    rows: [
      [['es', 'ES'], ['isch', 'ISCH']],
      [['fuenf', 'FÜNF'], ['zehn', 'ZEHN']],
      [['nach', 'NOCH'], ['vor', 'VOR'], ['halb', 'HALB']],
      [['dv', 'DREIVIERTEL']],
      [['h1', 'EIS'], ['h2', 'ZWEI'], ['h3', 'DRÜ']],
      [['h4', 'VIER'], ['h5', 'FÜNF']],
      [['h6', 'SECHS'], ['h7', 'SIEBE']],
      [['h8', 'ACHT'], ['h9', 'NÜN']],
      [['h10', 'ZEHN'], ['h11', 'ELF']],
      [['h12', 'ZWÖLF'], ['x', 'BÄCHLE']],
    ],
  },
  7,
);

export const palatinate = nachVor({
  id: 'pfl',
  name: 'Pfälzisch',
  filler: 'KQÜPXTRBJML',
  intro: ['es', 'is'],
  rows: [
    [['es', 'ES'], ['is', 'IS'], ['fuenf', 'FIMF']],
    [['zehn', 'ZEH'], ['viertel', 'VIERTEL']],
    [['zwanzig', 'ZWANZISCH']],
    [['nach', 'NOCH'], ['vor', 'VOR'], ['halb', 'HALB']],
    [['h1', 'ÄNS'], ['h2', 'ZWÄÄ'], ['h3', 'DREI']],
    [['h4', 'VIER'], ['h5', 'FIMF']],
    [['h6', 'SECHS'], ['h7', 'SIEWE']],
    [['h8', 'ACHT'], ['h9', 'NEIN']],
    [['h10', 'ZEH'], ['h11', 'ELF'], ['h12', 'ZWELF']],
    [['x', 'DUBBEGLAS']],
  ],
});

export const saarland = nachVor({
  id: 'saar',
  name: 'Saarländisch',
  filler: 'QXKÜPTRBJMÖ',
  intro: ['es', 'is'],
  // FÜNNEF and ZWANZISCH are long; one extra row keeps ES and IS apart.
  rows: [
    [['es', 'ES'], ['is', 'IS']],
    [['fuenf', 'FÜNNEF'], ['zehn', 'ZEHN']],
    [['viertel', 'VIERTEL']],
    [['zwanzig', 'ZWANZISCH']],
    [['nach', 'NOH'], ['vor', 'VOR'], ['halb', 'HALB']],
    [['h1', 'ÄÄNS'], ['h2', 'ZWOO']],
    [['h3', 'DREI'], ['h4', 'VIER']],
    [['h5', 'FÜNNEF'], ['h9', 'NEIN']],
    [['h6', 'SECHS'], ['h7', 'SIEWE']],
    [['h8', 'AACHT'], ['h10', 'ZEHN']],
    [['h11', 'ELF'], ['h12', 'ZWELF']],
  ],
});

export const ruhr = nachVor({
  id: 'ruhr',
  name: 'Ruhrpott',
  filler: 'XQKPÜTRBJMÖ',
  intro: ['et', 'is'],
  rows: [
    [['et', 'ET'], ['is', 'IS'], ['fuenf', 'FÜMF']],
    [['zehn', 'ZEHN'], ['viertel', 'VIERTEL']],
    [['zwanzig', 'ZWANZICH']],
    [['nach', 'NACH'], ['vor', 'VOR'], ['halb', 'HALB']],
    [['h1', 'EINS'], ['h2', 'ZWEI']],
    [['h3', 'DREI'], ['h4', 'VIER']],
    [['h5', 'FÜMF'], ['h6', 'SECHS']],
    [['h7', 'SIEBEN'], ['h8', 'ACHT']],
    [['h9', 'NEUN'], ['h10', 'ZEHN'], ['h11', 'ELF']],
    [['h12', 'ZWÖLF'], ['x', 'WATT']],
  ],
});

export const eastFrisian = nachVor({
  id: 'ofr',
  name: 'Ostfriesisch',
  filler: 'QXKBPRMJLÜZ',
  intro: ['dat', 'is'],
  rows: [
    [['dat', 'DAT'], ['is', 'IS'], ['fuenf', 'FIEF']],
    [['zehn', 'TEIN'], ['viertel', 'KWART']],
    [['zwanzig', 'TWINTIG']],
    [['nach', 'NA'], ['vor', 'VÖR'], ['halb', 'HALF']],
    [['h1', 'EEN'], ['h2', 'TWEE'], ['h3', 'DREE']],
    [['h4', 'VEER'], ['h5', 'FIEF']],
    [['h6', 'SES'], ['h7', 'SÖVEN']],
    [['h8', 'ACHT'], ['h9', 'NEGEN']],
    [['h10', 'TEIN'], ['h11', 'ELVEN']],
    [['h12', 'TWALF'], ['x', 'MOIN']],
  ],
});

// Koblenz says "zehn vür halwer" for :20, like older Rhineland speech; saves the long ZWANZISCH.
export const koblenz = build(
  {
    id: 'kob',
    name: 'Koblenzer Platt',
    filler: 'SCHÄNGELQXB',
    intro: ['et', 'es'],
    rows: [
      [['et', 'ET'], ['es', 'ES'], ['zehn', 'ZÄHN']],
      [['fuenf', 'FÜNNEF']],
      [['viertel', 'VEERTEL'], ['nach', 'NOH']],
      [['vor', 'VÜR'], ['halb', 'HALWER']],
      [['h1', 'EENS'], ['h2', 'ZWEE']],
      [['h3', 'DREI'], ['h4', 'VEER']],
      [['h5', 'FÜNNEF'], ['h9', 'NEUN']],
      [['h6', 'SECHS'], ['h7', 'SIEWE']],
      [['h8', 'AACHT'], ['h10', 'ZÄHN']],
      [['h11', 'ELF'], ['h12', 'ZWÖLF']],
    ],
  },
  (w) => {
    const { fuenf, zehn, viertel, nach, vor, halb } = w as Record<string, Cells>;
    return [
      [],
      [fuenf!, nach!],
      [zehn!, nach!],
      [viertel!, nach!],
      [zehn!, vor!, halb!],
      [fuenf!, vor!, halb!],
      [halb!],
      [fuenf!, nach!, halb!],
      [zehn!, nach!, halb!],
      [viertel!, vor!],
      [zehn!, vor!],
      [fuenf!, vor!],
    ];
  },
  4,
);
