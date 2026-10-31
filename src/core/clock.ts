import type { Cells, Grid } from './grid';
import type { Locale } from './locale';

export interface MinutesFirstOptions {
  id: string;
  name: string;
  lang: string;
  grid: Grid;
  /** "ES ISCH", "KLOKKEN ER" … shown when the intro setting is on. */
  intro: Cells[];
  /** Index = hour % 12, so index 0 is twelve. */
  hours: Cells[];
  /** One phrase per five minutes, placed before the hour ("VIERTEL", "FÜNF VOR HALB"). */
  minutes: Cells[][];
  /** From this five-minute step on, the phrase refers to the next hour ("halb vier" at 3:30). */
  nextHourFrom: number;
  /** Words after the hour on the full hour only, e.g. Dutch "UUR". */
  onTheHour?: Cells[];
  /** Words after the hour, e.g. Berlin "WA"; only with the intro. */
  outro?: Cells[];
}

/** Languages and dialects that say the minutes first and the hour last ("viertel vier", "fem i halv tre"). */
export function minutesFirst(o: MinutesFirstOptions): Locale {
  return {
    id: o.id,
    name: o.name,
    lang: o.lang,
    grid: o.grid,
    words: [...o.intro, ...o.minutes.flat(1), ...o.hours, ...(o.onTheHour ?? []), ...(o.outro ?? [])].filter(
      (word) => word.length > 0,
    ),
    frame(now, { intro }) {
      const step = Math.floor(now.getMinutes() / 5);
      const hour = o.hours[(now.getHours() + (step >= o.nextHourFrom ? 1 : 0)) % 12]!;
      const words = [
        ...(intro ? o.intro : []),
        ...o.minutes[step]!,
        hour,
        ...(step === 0 ? (o.onTheHour ?? []) : []),
        ...(intro ? (o.outro ?? []) : []),
      ];
      return { words, dots: now.getMinutes() % 5 };
    },
  };
}
