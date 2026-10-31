import { describe, expect, it } from 'vitest';
import { cycle } from './settings';

describe('cycle', () => {
  const list = ['a', 'b', 'c'] as const;

  it('steps forward and wraps', () => {
    expect(cycle(list, 'b')).toBe('c');
    expect(cycle(list, 'c')).toBe('a');
  });

  it('steps backward and wraps', () => {
    expect(cycle(list, 'b', -1)).toBe('a');
    expect(cycle(list, 'a', -1)).toBe('c');
  });
});
