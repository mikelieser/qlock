import { createGrid, type Cells } from '../core/grid';
import type { Locale } from '../core/locale';

// Kanji are square, so a small 6×6 grid fits the face well. Numbers are built from single digits
// in reading order (十 + 一 = 11, 二 + 十 + 五 = 25); the last row is weekday kanji as filler.
const grid = createGrid(['今は午前後十', '一二三四五六', '七八九時二三', '四五十五分半', 'です。日月火', '水木金土曜日']);

const at = (row: number, col: number, text: string) => grid.word(row, col, text);

const w = {
  ima: at(0, 0, '今は'),
  am: [...at(0, 2, '午'), ...at(0, 3, '前')],
  pm: [...at(0, 2, '午'), ...at(0, 4, '後')],
  ji: at(2, 3, '時'),
  fun: at(3, 4, '分'),
  han: at(3, 5, '半'),
  desu: at(4, 0, 'です'),
};

const hourTen = at(0, 5, '十');
const hourDigits = [...'一二三四五六七八九'].map((d, i) => at(i < 6 ? 1 : 2, i < 6 ? i : i - 6, d));

const minuteTens: Record<number, Cells> = { 2: at(2, 4, '二'), 3: at(2, 5, '三'), 4: at(3, 0, '四'), 5: at(3, 1, '五') };
const minuteTen = at(3, 2, '十');
const minuteFive = at(3, 3, '五');

/** 1 → 一, 10 → 十, 11 → 十一, 12 → 十二. */
function hourCells(h12: number): Cells {
  if (h12 < 10) return hourDigits[h12 - 1]!;
  return [...hourTen, ...(h12 > 10 ? hourDigits[h12 - 11]! : [])];
}

/** 5 → 五分, 25 → 二十五分, 30 → 半. */
function minuteCells(m: number): Cells[] {
  if (m === 0) return [];
  if (m === 30) return [w.han];
  const tens = Math.floor(m / 10);
  const parts: Cells[] = [];
  if (tens >= 2) parts.push(minuteTens[tens]!);
  if (tens >= 1) parts.push(minuteTen);
  if (m % 10 === 5) parts.push(minuteFive);
  return [...parts, w.fun];
}

const text = (cells: Cells[]) => cells.flat().map((c) => grid.chars[c]).join('');

export const ja: Locale = {
  id: 'ja',
  name: 'Japanisch',
  lang: 'ja-JP',
  grid,
  words: [w.ima, w.am, w.pm, w.ji, w.fun, w.han, w.desu, hourTen, ...hourDigits, ...Object.values(minuteTens), minuteTen, minuteFive],
  frame(now, { intro }) {
    const hours = now.getHours();
    const m = Math.floor(now.getMinutes() / 5) * 5;
    const h12 = hours % 12 || 12;
    const words = [
      ...(intro ? [w.ima] : []),
      hours < 12 ? w.am : w.pm,
      hourCells(h12),
      w.ji,
      ...minuteCells(m),
      w.desu,
    ];
    // Japanese is written without spaces between words.
    return { words, dots: now.getMinutes() % 5, title: text(words) };
  },
};
