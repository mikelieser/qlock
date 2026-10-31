/** Linear cell indices into a grid, row by row, 0-based. */
export type Cells = readonly number[];

export interface Grid {
  rows: number;
  cols: number;
  chars: readonly string[];
  /** Cells spelling `text` from row/col onwards. Throws if the grid doesn't spell it there. */
  word(row: number, col: number, text: string): Cells;
}

export function createGrid(lines: readonly string[]): Grid {
  // Spread instead of length/index so multi-byte letters (Ü, Σ, ø) count as one cell.
  const split = lines.map((line) => [...line]);
  const cols = split[0]?.length ?? 0;
  split.forEach((line, i) => {
    if (line.length !== cols) throw new Error(`Grid row ${i} has ${line.length} cells, expected ${cols}`);
  });
  const chars = split.flat();

  return {
    rows: split.length,
    cols,
    chars,
    word(row, col, text) {
      const letters = [...text];
      const start = row * cols + col;
      const cells = letters.map((_, i) => start + i);
      const found = cells.map((c) => chars[c] ?? '').join('');
      if (col + letters.length > cols || found !== text) {
        throw new Error(`Expected "${text}" at row ${row}, col ${col}, found "${found}"`);
      }
      return cells;
    },
  };
}
