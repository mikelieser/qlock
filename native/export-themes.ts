// Exports every theme's CSS custom properties from src/styles.css, with the cascade applied and var()
// resolved, so the native renderer reads the same values as the browser. The web CSS stays the single
// source of truth; only effects that CSS does with extra selectors (spiders, rays …) are coded natively.
// Usage: bun native/export-themes.ts
import { THEMES } from '../src/settings';

const css = (await Bun.file(new URL('../src/styles.css', import.meta.url)).text()).replace(/\/\*[\s\S]*?\*\//g, '');

interface Rule {
  selectors: string[];
  decls: [string, string][];
}

/** Top-level rules; @-blocks (keyframes, media) are skipped. */
function parseRules(text: string): Rule[] {
  const rules: Rule[] = [];
  let i = 0;
  while (i < text.length) {
    const open = text.indexOf('{', i);
    if (open < 0) break;
    const prelude = text.slice(i, open).trim();
    let depth = 1;
    let j = open + 1;
    while (j < text.length && depth > 0) {
      if (text[j] === '{') depth++;
      else if (text[j] === '}') depth--;
      j++;
    }
    const body = text.slice(open + 1, j - 1);
    i = j;
    if (prelude.startsWith('@')) continue;
    rules.push({ selectors: splitTop(prelude, ',').map((s) => s.trim().replace(/\s+/g, ' ')), decls: parseDecls(body) });
  }
  return rules;
}

/** Splits at `sep` outside parentheses and quotes. */
function splitTop(text: string, sep: string): string[] {
  const parts: string[] = [];
  let depth = 0;
  let quote = '';
  let start = 0;
  for (let k = 0; k < text.length; k++) {
    const c = text[k]!;
    if (quote) {
      if (c === quote) quote = '';
    } else if (c === '"' || c === "'") quote = c;
    else if (c === '(') depth++;
    else if (c === ')') depth--;
    else if (c === sep && depth === 0) {
      parts.push(text.slice(start, k));
      start = k + 1;
    }
  }
  parts.push(text.slice(start));
  return parts.filter((p) => p.trim() !== '');
}

function parseDecls(body: string): [string, string][] {
  return splitTop(body, ';')
    .map((d) => d.trim())
    .filter((d) => d.startsWith('--'))
    .map((d) => {
      const colon = d.indexOf(':');
      return [d.slice(0, colon).trim(), d.slice(colon + 1).trim().replace(/\s+/g, ' ')] as [string, string];
    });
}

const rules = parseRules(css);

function collect(match: (selector: string) => boolean): Map<string, string> {
  const vars = new Map<string, string>();
  for (const rule of rules) {
    if (rule.selectors.some(match)) for (const [k, v] of rule.decls) vars.set(k, v);
  }
  return vars;
}

/** Substitutes var(--x, fallback) recursively. */
function resolve(value: string, vars: Map<string, string>, seen = new Set<string>()): string {
  let out = '';
  let k = 0;
  while (k < value.length) {
    const at = value.indexOf('var(', k);
    if (at < 0) {
      out += value.slice(k);
      break;
    }
    out += value.slice(k, at);
    let depth = 1;
    let j = at + 4;
    while (j < value.length && depth > 0) {
      if (value[j] === '(') depth++;
      else if (value[j] === ')') depth--;
      j++;
    }
    const inner = value.slice(at + 4, j - 1);
    const [name, ...rest] = splitTop(inner, ',');
    const key = name!.trim();
    const fallback = rest.join(',').trim();
    if (vars.has(key) && !seen.has(key)) out += resolve(vars.get(key)!, vars, new Set([...seen, key]));
    else out += resolve(fallback, vars, seen);
    k = j;
  }
  return out.trim();
}

const defaults = collect((s) => s === ':root' || s === '[data-theme]');

const themes = Object.fromEntries(
  THEMES.map((id) => {
    const own = collect((s) => s === `[data-theme='${id}']`);
    const vars = new Map([...defaults, ...own]);
    const resolved = Object.fromEntries([...vars.keys()].map((k) => [k.slice(2), resolve(vars.get(k)!, vars)]));
    // Rules on the dots only (e.g. a glow in the dot's own colour): --dot-on stays as `currentcolor`.
    const dotOwn = collect((s) => s === `[data-theme='${id}'] .dot`);
    const dotVars = new Map([...vars, ...dotOwn, ['--dot-on', 'currentcolor']]);
    const dot = Object.fromEntries([...dotOwn.keys()].map((k) => [k.slice(2), resolve(dotOwn.get(k)!, dotVars)]));
    if (!dot['dot-glow']) dot['dot-glow'] = resolve(vars.get('--dot-glow') ?? 'none', new Map([...vars, ['--dot-on', 'currentcolor']]));
    return [id, { vars: resolved, dot }];
  }),
);

await Bun.write(new URL('./data/themes.json', import.meta.url), JSON.stringify({ themes }, null, 1));
console.log(`${Object.keys(themes).length} themes exported`);
