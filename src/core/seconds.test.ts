import { describe, expect, it } from 'vitest';
import { createGrid } from './grid';
import { secondsFrame } from './seconds';

const grid = createGrid(Array.from({ length: 10 }, () => 'ABCDEFGHIJK'));

const render = (seconds: number) => {
  const lit = new Set(secondsFrame(grid, seconds)!.words.flat());
  return Array.from({ length: grid.rows }, (_, r) =>
    Array.from({ length: grid.cols }, (_, c) => (lit.has(r * grid.cols + c) ? '#' : '.')).join(''),
  );
};

describe('secondsFrame', () => {
  it('draws both digits side by side, vertically centred', () => {
    expect(render(7)).toEqual([
      '...........',
      '.###..#####',
      '#...#.....#',
      '#...#....#.',
      '#...#...#..',
      '#...#..#...',
      '#...#..#...',
      '.###...#...',
      '...........',
      '...........',
    ]);
  });

  it('titles the frame with the padded seconds and no dots', () => {
    expect(secondsFrame(grid, 42)).toMatchObject({ title: '42 Sekunden', dots: 0 });
  });

  it('gives up on grids that are too small', () => {
    expect(secondsFrame(createGrid(['TICK', 'TACK']), 5)).toBeUndefined();
  });
});
