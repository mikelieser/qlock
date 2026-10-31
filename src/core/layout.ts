import { createGrid, type Cells, type Grid } from './grid';

/** A row of named words, or a literal row of exactly `width` letters (fillers, regional words). */
export type LayoutRow = readonly (readonly [key: string, text: string])[] | string;

/**
 * Builds a grid from words per row, so dialect grids can be written as word lists instead of hand-counted
 * letter strings. Words are separated by one filler letter where the row has room, otherwise they touch.
 */
export function layoutGrid(rows: readonly LayoutRow[], filler: string, width = 11): { grid: Grid; w: Record<string, Cells> } {
  let next = 0;
  const pad = () => filler[next++ % filler.length]!;
  const placed: { key: string; text: string; row: number; col: number }[] = [];

  const lines = rows.map((row, r) => {
    if (typeof row === 'string') return row;
    const total = row.reduce((sum, [, text]) => sum + [...text].length, 0);
    const gap = total + row.length - 1 <= width ? 1 : 0;
    if (total + gap * (row.length - 1) > width) throw new Error(`Row ${r} does not fit: ${row.map(([, t]) => t).join(' ')}`);
    let line = '';
    row.forEach(([key, text], i) => {
      if (i > 0 && gap) line += pad();
      placed.push({ key, text, row: r, col: [...line].length });
      line += text;
    });
    while ([...line].length < width) line += pad();
    return line;
  });

  const grid = createGrid(lines);
  const w = Object.fromEntries(placed.map(({ key, text, row, col }) => [key, grid.word(row, col, text)]));
  return { grid, w };
}
