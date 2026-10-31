import type { Cells, Grid } from './grid';
import type { Frame } from './locale';

// 5×7 pixel digits. Pixels, not words, so this is the one place besides pictures that uses raw cells.
const DIGITS: readonly (readonly string[])[] = [
  ['.###.', '#...#', '#...#', '#...#', '#...#', '#...#', '.###.'],
  ['..#..', '.##..', '..#..', '..#..', '..#..', '..#..', '.###.'],
  ['.###.', '#...#', '....#', '...#.', '..#..', '.#...', '#####'],
  ['#####', '...#.', '..#..', '...#.', '....#', '#...#', '.###.'],
  ['...#.', '..##.', '.#.#.', '#..#.', '#####', '...#.', '...#.'],
  ['#####', '#....', '####.', '....#', '....#', '#...#', '.###.'],
  ['..##.', '.#...', '#....', '####.', '#...#', '#...#', '.###.'],
  ['#####', '....#', '...#.', '..#..', '.#...', '.#...', '.#...'],
  ['.###.', '#...#', '#...#', '.###.', '#...#', '#...#', '.###.'],
  ['.###.', '#...#', '#...#', '.####', '....#', '...#.', '.##..'],
];
const DIGIT_W = 5;
const DIGIT_H = 7;
const WIDTH = 2 * DIGIT_W + 1;

/** Two-digit seconds drawn as pixels into the grid (no dots), or undefined if the grid is too small. */
export function secondsFrame(grid: Grid, seconds: number): Frame | undefined {
  if (grid.cols < WIDTH || grid.rows < DIGIT_H) return undefined;
  const top = Math.floor((grid.rows - DIGIT_H) / 2);
  const left = Math.floor((grid.cols - WIDTH) / 2);

  const digit = (value: number, x: number): Cells =>
    DIGITS[value]!.flatMap((line, row) =>
      [...line].flatMap((pixel, col) => (pixel === '#' ? [(top + row) * grid.cols + left + x + col] : [])),
    );

  const s = Math.max(0, Math.min(59, Math.floor(seconds)));
  return {
    words: [digit(Math.floor(s / 10), 0), digit(s % 10, DIGIT_W + 1)],
    dots: 0,
    title: `${String(s).padStart(2, '0')} Sekunden`,
  };
}
