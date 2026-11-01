import { describe, expect, it } from 'vitest';
import { daylight, sunElevation } from './daylight';

// UTC times; Koblenz is UTC+1 in winter, UTC+2 in summer.
const at = (iso: string) => new Date(iso);

describe('daylight', () => {
  it('knows roughly where the sun is', () => {
    expect(sunElevation(at('2026-06-21T11:30:00Z'))).toBeGreaterThan(60);
    expect(sunElevation(at('2026-12-21T11:30:00Z'))).toBeLessThan(18);
    expect(sunElevation(at('2026-06-21T23:00:00Z'))).toBeLessThan(-10);
  });

  // Shared with native/core.
  it.each([
    ['2026-11-01T02:00:00Z', 'nightsurf'],
    ['2026-11-01T06:30:00Z', 'sunrise'], // sunrise in Koblenz ~7:15 local
    ['2026-11-01T11:00:00Z', 'lagoon'],
    ['2026-11-01T15:45:00Z', 'sunset'], // sunset ~17:05 local
    ['2026-06-21T03:30:00Z', 'sunrise'],
    ['2026-06-21T19:30:00Z', 'sunset'],
    ['2026-06-21T21:00:00Z', 'nightsurf'],
  ] as const)('%s is %s', (iso, theme) => {
    expect(daylight(at(iso))).toBe(theme);
  });
});
