import { minutesFirst } from '../core/clock';
import type { Cells } from '../core/grid';
import { layoutGrid, type LayoutRow } from '../core/layout';
import type { Locale } from '../core/locale';

/*
 * Tongue-in-cheek German faces: the same "zehn vor halb" counting as Koblenz, with their own opening and
 * closing words and something hidden in the filler letters.
 */

const HOURS = ['h12', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'h7', 'h8', 'h9', 'h10', 'h11'];

/** Shared middle: minute words, then the hours. */
const TIME: LayoutRow[] = [
  [
    ['zehn', 'ZEHN'],
    ['fuenf', 'FÜNF'],
  ],
  [
    ['viertel', 'VIERTEL'],
    ['nach', 'NACH'],
  ],
  [
    ['vor', 'VOR'],
    ['halb', 'HALB'],
  ],
  [
    ['h1', 'EINS'],
    ['h2', 'ZWEI'],
  ],
  [
    ['h3', 'DREI'],
    ['h4', 'VIER'],
  ],
  [
    ['h5', 'FÜNF'],
    ['h6', 'SECHS'],
  ],
  [
    ['h7', 'SIEBEN'],
    ['h8', 'ACHT'],
  ],
  [
    ['h9', 'NEUN'],
    ['h10', 'ZEHN'],
  ],
  [
    ['h11', 'ELF'],
    ['h12', 'ZWÖLF'],
  ],
];

interface FunSpec {
  id: string;
  name: string;
  before: LayoutRow[];
  after: LayoutRow[];
  filler: string;
  intro: string[];
  outro: string[];
}

function fun(spec: FunSpec): Locale {
  const { grid, w } = layoutGrid([...spec.before, ...TIME, ...spec.after], spec.filler);
  const { fuenf, zehn, viertel, nach, vor, halb } = w as Record<string, Cells>;
  return minutesFirst({
    id: spec.id,
    name: spec.name,
    lang: 'de-DE',
    grid,
    intro: spec.intro.map((k) => w[k]!),
    outro: spec.outro.map((k) => w[k]!),
    hours: HOURS.map((k) => w[k]!),
    minutes: [
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
    ],
    nextHourFrom: 4,
  });
}

/** For kids: "Guck mal, es ist … juhu!", sweets in the filler letters. */
export const kids = fun({
  id: 'kids',
  name: 'Kinderuhr',
  before: [
    [
      ['guck', 'GUCK'],
      ['mal', 'MAL'],
    ],
    [
      ['es', 'ES'],
      ['ist', 'IST'],
    ],
  ],
  after: [[['juhu', 'JUHU']]],
  filler: 'EISKEKSTEDDYBONBON',
  intro: ['guck', 'mal', 'es', 'ist'],
  outro: ['juhu'],
});

/** For bankers: "Zeit ist Geld: … bullish." */
export const boerse = fun({
  id: 'boerse',
  name: 'Börse',
  before: [
    [
      ['zeit', 'ZEIT'],
      ['ist', 'IST'],
      ['geld', 'GELD'],
    ],
  ],
  after: [[['bullish', 'BULLISH']]],
  filler: 'DAXBULLEBÄRCASH',
  intro: ['zeit', 'ist', 'geld'],
  outro: ['bullish'],
});

/** Spiritual: "Jetzt ist … om." */
export const zen = fun({
  id: 'zen',
  name: 'Zen',
  before: [
    [
      ['jetzt', 'JETZT'],
      ['ist', 'IST'],
    ],
  ],
  after: [[['om', 'OM']]],
  filler: 'ATMELIEBESEINRUHE',
  intro: ['jetzt', 'ist'],
  outro: ['om'],
});

/** Rude: "Scheiße, es ist schon … Alter." */
export const derb = fun({
  id: 'derb',
  name: 'Derb',
  before: [
    [['scheisse', 'SCHEISSE']],
    [
      ['es', 'ES'],
      ['ist', 'IST'],
      ['schon', 'SCHON'],
    ],
  ],
  after: [[['alter', 'ALTER']]],
  filler: 'MISTMANNOKACKE',
  intro: ['scheisse', 'es', 'ist', 'schon'],
  outro: ['alter'],
});

/** For nerds: "sudo es ist … EOF", a bug and the answer in the filler letters. */
export const nerd = fun({
  id: 'nerd',
  name: 'Nerd',
  before: [
    [
      ['sudo', 'SUDO'],
      ['es', 'ES'],
      ['ist', 'IST'],
    ],
  ],
  // EOF closes the bottom-right corner, after the commands it ends.
  after: [
    [
      ['tools', 'VIMGIT42'],
      ['eof', 'EOF'],
    ],
  ],
  filler: '42BUGNULLVIMGIT',
  intro: ['sudo', 'es', 'ist'],
  outro: ['eof'],
});

/** Youth slang 2025/26: "Digga, es ist … no cap", aura, sus, bro and 67 in the filler letters. */
export const digga = fun({
  id: 'digga',
  name: 'Digga',
  before: [
    [
      ['digga', 'DIGGA'],
      ['es', 'ES'],
      ['ist', 'IST'],
    ],
  ],
  after: [
    [
      ['no', 'NO'],
      ['cap', 'CAP'],
    ],
  ],
  filler: 'AURASUSBRO67LOWKEY',
  intro: ['digga', 'es', 'ist'],
  outro: ['no', 'cap'],
});

/** The Hitchhiker's Guide: "Keine Panik", the towel and the answer to everything. */
export const anhalter = fun({
  id: 'anhalter',
  name: 'Anhalter',
  before: [
    [
      ['keine', 'KEINE'],
      ['panik', 'PANIK'],
    ],
  ],
  after: [
    [
      ['tuch', 'HANDTUCH'],
      ['answer', '42'],
    ],
  ],
  filler: 'BABELFISCHDNTARTHUR',
  intro: ['keine', 'panik'],
  outro: ['answer'],
});
