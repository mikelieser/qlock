import { describe, expect, it } from 'vitest';
import { easter, festival } from './festival';

const on = (y: number, m: number, d: number) => festival(new Date(y, m - 1, d));

describe('festival', () => {
  it('finds Easter', () => {
    expect(easter(2026)).toEqual([4, 5]);
    expect(easter(2027)).toEqual([3, 28]);
    expect(easter(2025)).toEqual([4, 20]);
  });

  // Shared with native/core so both sides agree on every date.
  it.each([
    [2026, 1, 1, 'neujahr'],
    [2026, 1, 6, 'neujahr'],
    [2026, 1, 7, undefined],
    [2026, 2, 12, 'karneval'], // Weiberfastnacht
    [2026, 2, 16, 'karneval'], // Rosenmontag
    [2026, 2, 17, 'karneval'],
    [2026, 2, 18, undefined], // Aschermittwoch
    [2026, 3, 29, 'ostern'], // Palmsonntag
    [2026, 4, 6, 'ostern'], // Ostermontag
    [2026, 4, 7, undefined],
    [2026, 10, 23, undefined],
    [2026, 10, 31, 'halloween'],
    [2026, 11, 28, undefined],
    [2026, 11, 29, 'weihnachten'], // 1. Advent 2026
    [2026, 12, 26, 'weihnachten'],
    [2026, 12, 27, 'silvester'],
    [2026, 12, 31, 'silvester'],
    [2027, 11, 28, 'weihnachten'], // 1. Advent 2027
    [2027, 11, 27, undefined],
    [2023, 12, 3, 'weihnachten'], // 24.12.2023 was a Sunday
    [2023, 12, 2, undefined],
  ] as const)('%i-%i-%i is %s', (y, m, d, expected) => {
    expect(on(y, m, d)).toBe(expected);
  });
});
