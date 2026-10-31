import { describe, expect, it } from 'vitest';
import { formatDate } from './date';

describe('formatDate', () => {
  const day = new Date(2026, 9, 31, 12, 0);

  it('writes German weekday and date', () => {
    expect(formatDate(day, 'de-DE')).toBe('Sa · 31. Oktober');
  });

  it('writes English weekday and date', () => {
    expect(formatDate(day, 'en-GB')).toBe('Sat · 31 October');
  });
});
