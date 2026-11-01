/**
 * The tab icon follows the theme: a small face in the theme's colours with a 5×5 grid of cells, some lit
 * like words on the clock, built from the theme's own CSS custom properties.
 */

const COLOR = /#[0-9a-f]{3,8}\b|rgba?\([^)]*\)|hsla?\([^)]*\)/gi;

/** Alpha of a CSS colour (1 unless it says otherwise). */
function alpha(c: string): number {
  const m = c.match(/\/\s*([\d.]+)(%?)\s*\)$/) ?? c.match(/^(?:rgba|hsla)\(.*,\s*([\d.]+)(%?)\s*\)$/);
  if (m) return m[2] ? Number(m[1]) / 100 : Number(m[1]);
  if (/^#[0-9a-f]{8}$/i.test(c)) return parseInt(c.slice(7), 16) / 255;
  return 1;
}

/** Mostly opaque colours of a CSS value (overlays and glows left out); falls back to `fallback`. */
function colors(value: string, fallback: string): string[] {
  const solid = (value.match(COLOR) ?? []).filter((c) => alpha(c) >= 0.6);
  return solid.length > 0 ? solid : [fallback];
}

/** A tiny letter grid: a few cells lit like words in a sentence. */
const PATTERN = ['11011', '00000', '01110', '00000', '11100'];

export function themeIcon(root: HTMLElement): string {
  const css = getComputedStyle(root);
  const v = (name: string) => css.getPropertyValue(name).trim();
  // The face's base layer comes last in its background list: use its first and last colour.
  const faceBg = v('--face-bg');
  const base = faceBg.slice(Math.max(0, faceBg.lastIndexOf('gradient(')));
  const face = colors(base, colors(v('--page-bg'), '#000')[0]!);
  const [to, from] = [face[0]!, face[face.length - 1]!];
  const letter = colors(v('--char-on'), '#fff')[0]!;
  const block = colors(v('--char-on-bg'), '')[0]!;
  // Lit cells in the letter colour (Formrausch: its yellow block), the rest like unlit letters.
  const lit = block || letter;
  const dim = v('--char-off') || 'rgb(255 255 255 / 0.15)';
  const cells = PATTERN.flatMap((row, r) =>
    [...row].map((on, c) => `<rect x='${9 + c * 10}' y='${9 + r * 10}' width='6' height='6' rx='1.5' fill='${on === '1' ? lit : dim}'/>`),
  );
  const svg = `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 64 64'>
<defs><linearGradient id='g' x1='0' y1='0' x2='1' y2='1'><stop offset='0' stop-color='${to}'/><stop offset='1' stop-color='${from}'/></linearGradient></defs>
<rect width='64' height='64' rx='14' fill='url(#g)'/>
${cells.join('')}
</svg>`;
  return `data:image/svg+xml,${encodeURIComponent(svg)}`;
}

export function setFavicon(href: string): void {
  let link = document.querySelector<HTMLLinkElement>("link[rel='icon']");
  if (!link) {
    link = document.createElement('link');
    link.rel = 'icon';
    document.head.appendChild(link);
  }
  link.type = 'image/svg+xml';
  link.href = href;
}
