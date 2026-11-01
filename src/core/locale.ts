import type { Cells, Grid } from './grid';

export interface Frame {
  /** Lit words in reading order; the order drives the light-up stagger. */
  words: readonly Cells[];
  /** Corner dots lit, 0–4. */
  dots: number;
  /** Overrides the text derived from `words`, e.g. for picture frames. */
  title?: string;
}

export interface FrameOptions {
  intro: boolean;
}

export interface Locale {
  id: string;
  name: string;
  /** BCP 47 tag for dates and other text outside the grid. */
  lang: string;
  grid: Grid;
  /** Every word the locale can show, for the "all" mode. */
  words: readonly Cells[];
  frame(now: Date, options: FrameOptions): Frame;
  /** Words that light on one day of the year (birthdays, Towel Day); `frame` already includes them. */
  dated?: readonly Dated[];
}

export interface Dated {
  month: number;
  day: number;
  words: readonly Cells[];
}

export function frameText(grid: Grid, frame: Frame): string {
  return frame.title ?? frame.words.map((word) => word.map((c) => grid.chars[c]).join('')).join(' ');
}

export function sameFrame(a: Frame, b: Frame): boolean {
  return a.dots === b.dots && a.title === b.title && a.words.flat().join() === b.words.flat().join();
}
