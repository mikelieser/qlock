import type { Cells } from './grid';

export interface GermanWords {
  es: Cells;
  ist: Cells;
  fuenf: Cells;
  zehn: Cells;
  viertel: Cells;
  zwanzig: Cells;
  vor: Cells;
  nach: Cells;
  halb: Cells;
  uhr: Cells;
  /** "EIN" for "es ist ein Uhr". */
  ein: Cells;
  /** Index = hour % 12, so index 0 is ZWÖLF. */
  hours: readonly Cells[];
}

export function germanTime(w: GermanWords, now: Date, intro: boolean): Cells[] {
  const h = now.getHours() % 12;
  const hour = w.hours[h]!;
  const next = w.hours[(h + 1) % 12]!;

  const phrases: Cells[][] = [
    [h === 1 ? w.ein : hour, w.uhr],
    [w.fuenf, w.nach, hour],
    [w.zehn, w.nach, hour],
    [w.viertel, w.nach, hour],
    [w.zwanzig, w.nach, hour],
    [w.fuenf, w.vor, w.halb, next],
    [w.halb, next],
    [w.fuenf, w.nach, w.halb, next],
    [w.zwanzig, w.vor, next],
    [w.viertel, w.vor, next],
    [w.zehn, w.vor, next],
    [w.fuenf, w.vor, next],
  ];

  const intros = intro ? [w.es, w.ist] : [];
  return [...intros, ...phrases[Math.floor(now.getMinutes() / 5)]!];
}

export function allGermanWords(w: GermanWords): Cells[] {
  const { hours, ein: _ein, ...rest } = w;
  return [...Object.values(rest), ...hours];
}
