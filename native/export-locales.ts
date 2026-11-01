// Exports every locale as plain data for the native renderer, so the TypeScript locales stay the single
// source of truth for language logic. Each minute of the day (with and without intro) points at a frame.
// Usage: bun native/export-locales.ts
import { LOCALE_GROUPS } from '../src/locales';
import type { Frame } from '../src/core/locale';

const MINUTES = 1440;

const locales = LOCALE_GROUPS.flatMap((group) =>
  group.locales.map((locale) => {
    const words: number[][] = [];
    const wordIndex = new Map<string, number>();
    const frames: { words: number[]; title?: string }[] = [];
    const frameIndex = new Map<string, number>();

    const wordId = (cells: readonly number[]) => {
      const key = cells.join(',');
      if (!wordIndex.has(key)) {
        wordIndex.set(key, words.length);
        words.push([...cells]);
      }
      return wordIndex.get(key)!;
    };
    const frameId = (frame: Frame) => {
      const entry = { words: frame.words.map(wordId), ...(frame.title ? { title: frame.title } : {}) };
      const key = JSON.stringify(entry);
      if (!frameIndex.has(key)) {
        frameIndex.set(key, frames.length);
        frames.push(entry);
      }
      return frameIndex.get(key)!;
    };

    const at = (m: number) => new Date(2026, 0, 5, Math.floor(m / 60), m % 60);
    const minutes = [false, true].map((intro) =>
      Array.from({ length: MINUTES }, (_, m) => frameId(locale.frame(at(m), { intro }))),
    );
    const dots = Array.from({ length: MINUTES }, (_, m) => {
      const [a, b] = [false, true].map((intro) => locale.frame(at(m), { intro }).dots);
      if (a !== b) throw new Error(`${locale.id}: dots depend on the intro at minute ${m}`);
      return a;
    });

    return {
      id: locale.id,
      name: locale.name,
      lang: locale.lang,
      group: group.name,
      rows: locale.grid.rows,
      cols: locale.grid.cols,
      chars: locale.grid.chars,
      words,
      frames,
      // [without intro, with intro], one frame index per minute of the day
      minutes,
      dots,
    };
  }),
);

await Bun.write(new URL('./data/locales.json', import.meta.url), JSON.stringify({ locales }));
console.log(`${locales.length} locales exported`);
