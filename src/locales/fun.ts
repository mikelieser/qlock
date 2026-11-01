import { minutesFirst } from '../core/clock';
import type { Cells } from '../core/grid';
import { layoutGrid, type LayoutRow } from '../core/layout';
import type { Dated, Locale } from '../core/locale';

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
  /** Occasion words lit during a time of day, `[key, 'HH:MM' from, 'HH:MM' until]`; may wrap past midnight. */
  moments?: [key: string, from: string, until: string][];
  /** Words lit all day on a date, `[key, month, day]`. */
  dated?: [key: string, month: number, day: number][];
  width?: number;
}

const minuteOfDay = (hhmm: string) => {
  const [h, m] = hhmm.split(':').map(Number);
  return h! * 60 + m!;
};

function fun(spec: FunSpec): Locale {
  const { grid, w } = layoutGrid([...spec.before, ...TIME, ...spec.after], spec.filler, spec.width);
  const moments = (spec.moments ?? []).map(([key, from, until]) => ({
    word: w[key]!,
    from: minuteOfDay(from),
    until: minuteOfDay(until),
  }));
  const dated: Dated[] = (spec.dated ?? []).map(([key, month, day]) => ({ month, day, words: [w[key]!] }));
  const base = clock(spec, grid, w);
  if (moments.length === 0 && dated.length === 0) return base;
  return {
    ...base,
    words: [...base.words, ...moments.map((m) => m.word), ...dated.flatMap((d) => d.words)],
    dated,
    frame(now, options) {
      const frame = base.frame(now, options);
      const m = now.getHours() * 60 + now.getMinutes();
      const lit = moments.filter(({ from, until }) => (from <= until ? m >= from && m < until : m >= from || m < until));
      const today = dated.filter((d) => d.month === now.getMonth() + 1 && d.day === now.getDate());
      return { ...frame, words: [...frame.words, ...lit.map((x) => x.word), ...today.flatMap((d) => d.words)] };
    },
  };
}

function clock(spec: FunSpec, grid: Locale['grid'], w: Record<string, Cells>): Locale {
  const { fuenf, zehn, viertel, nach, vor, halb } = w;
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
  after: [
    [
      ['juhu', 'JUHU'],
      ['heia', 'HEIA'],
    ],
  ],
  filler: 'EISKEKSTEDDYBONBON',
  intro: ['guck', 'mal', 'es', 'ist'],
  outro: ['juhu'],
  moments: [['heia', '19:00', '07:00']],
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
  after: [
    [
      ['om', 'OM'],
      ['teezeit', 'TEEZEIT'],
    ],
  ],
  filler: 'ATMELIEBESEINRUHE',
  intro: ['jetzt', 'ist'],
  outro: ['om'],
  moments: [['teezeit', '16:00', '17:00']],
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
  after: [
    [
      ['alter', 'ALTER'],
      ['bier', 'BIER'],
    ],
  ],
  filler: 'MISTMANNOKACKE',
  intro: ['scheisse', 'es', 'ist', 'schon'],
  outro: ['alter'],
  moments: [['bier', '18:00', '02:00']],
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
      ['lit', 'LIT'],
    ],
  ],
  filler: 'AURASUSBRO67LOWKEY',
  intro: ['digga', 'es', 'ist'],
  outro: ['no', 'cap'],
  moments: [['lit', '22:00', '04:00']],
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
  // Towel Day.
  dated: [['tuch', 5, 25]],
});

/** Deutsche Bahn: "Es ist ca. … +10 Min", disruptions in the filler letters. */
export const bahn = fun({
  id: 'bahn',
  name: 'Bahn',
  before: [
    [
      ['es', 'ES'],
      ['ist', 'IST'],
      ['ca', 'CA.'],
    ],
  ],
  after: [
    [
      ['delay', '+10'],
      ['min', 'MIN'],
    ],
  ],
  filler: 'STÖRUNGSEVGLEISICE',
  intro: ['es', 'ist', 'ca'],
  outro: ['delay', 'min'],
});

/** Officialese: "Amtlich ist …", and from half past three it's closing time. */
export const amt = fun({
  id: 'amt',
  name: 'Amt',
  before: [
    [
      ['amtlich', 'AMTLICH'],
      ['ist', 'IST'],
    ],
  ],
  after: [[['feierabend', 'FEIERABEND']]],
  filler: 'AKTENSTEMPELFORMULAR',
  intro: ['amtlich', 'ist'],
  outro: [],
  moments: [['feierabend', '15:30', '08:00']],
});

/** Pirates: "Ahoi, es ist … arrr", rum from six. */
export const pirat = fun({
  id: 'pirat',
  name: 'Pirat',
  before: [
    [
      ['ahoi', 'AHOI'],
      ['es', 'ES'],
      ['ist', 'IST'],
    ],
  ],
  after: [
    [
      ['arrr', 'ARRR'],
      ['rum', 'RUM'],
    ],
  ],
  filler: 'SCHATZKAPERNMEERHAI',
  intro: ['ahoi', 'es', 'ist'],
  outro: ['arrr'],
  moments: [['rum', '18:00', '02:00']],
});

/** Football: a goal every full hour, final whistle after the Saturday games. */
export const fussball = fun({
  id: 'fussball',
  name: 'Fußball',
  before: [
    [
      ['es', 'ES'],
      ['ist', 'IST'],
    ],
  ],
  after: [
    [
      ['abpfiff', 'ABPFIFF'],
      ['tor', 'TOR'],
    ],
  ],
  filler: 'BALLELFMETERECKEABSEITS',
  intro: ['es', 'ist'],
  outro: [],
  moments: [
    ...Array.from({ length: 24 }, (_, h): [string, string, string] => ['tor', `${h}:00`, `${h}:01`]),
    ['abpfiff', '17:20', '17:30'],
  ],
});

/** `[key, month, day]` per family member; their name and JUHU light all day. */
const BIRTHDAYS: [key: string, month: number, day: number][] = [];

/** The family: everyone's name hidden in the grid, lit on their birthday. */
export const familie = fun({
  id: 'familie',
  name: 'Familie',
  width: 12,
  before: [
    [
      ['es', 'ES'],
      ['ist', 'IST'],
      ['mike', 'MIKE'],
    ],
  ],
  after: [
    [
      ['katja', 'KATJA'],
      ['henry', 'HENRY'],
    ],
    [
      ['susanna', 'SUSANNA'],
      ['juhu', 'JUHU'],
    ],
  ],
  filler: 'HERZLICHLIEBFAMILIE',
  intro: ['es', 'ist'],
  outro: [],
  dated: BIRTHDAYS.flatMap(([key, month, day]) => [
    [key, month, day],
    ['juhu', month, day],
  ]),
});
