import { describe, expect, it } from 'vitest';
import { createGrid } from './grid';

describe('createGrid', () => {
  it('counts multi-byte letters as one cell', () => {
    const grid = createGrid(['AÜB', 'ΣøC']);
    expect(grid.cols).toBe(3);
    expect(grid.word(1, 0, 'Σø')).toEqual([3, 4]);
  });

  it('rejects ragged rows', () => {
    expect(() => createGrid(['ABC', 'AB'])).toThrow(/row 1/);
  });

  it('rejects a word the grid does not spell', () => {
    const grid = createGrid(['ABC']);
    expect(() => grid.word(0, 1, 'BX')).toThrow(/found "BC"/);
    expect(() => grid.word(0, 2, 'CD')).toThrow();
  });
});
