/**
 * The tab icon follows the theme: a small face in the theme's colours with a lit "Q" and the four corner
 * dots, built from the theme's own CSS custom properties.
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

export function themeIcon(root: HTMLElement): string {
  const css = getComputedStyle(root);
  const v = (name: string) => css.getPropertyValue(name).trim();
  // The face's base layer comes last in its background list: use its first and last colour.
  const faceBg = v('--face-bg');
  const base = faceBg.slice(Math.max(0, faceBg.lastIndexOf('gradient(')));
  const face = colors(base, colors(v('--page-bg'), '#000')[0]!);
  const [to, from] = [face[0]!, face[face.length - 1]!];
  const letter = colors(v('--char-on'), '#fff')[0]!;
  const block = colors(v('--char-on-bg'), 'transparent')[0]!;
  const dots = [1, 2, 3, 4].map((n) => colors(v(`--dot-${n}`), letter)[0]!);
  const font = v('--font').includes('Amatic') ? "'Amatic SC', sans-serif" : "'Inter Variable', Helvetica, Arial, sans-serif";
  const svg = `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 64 64'>
<defs><linearGradient id='g' x1='0' y1='0' x2='1' y2='1'><stop offset='0' stop-color='${to}'/><stop offset='1' stop-color='${from}'/></linearGradient></defs>
<rect width='64' height='64' rx='14' fill='url(#g)'/>
${block !== 'transparent' ? `<rect x='17' y='15' width='30' height='34' fill='${block}'/>` : ''}
<text x='32' y='44' text-anchor='middle' font-family="${font}" font-weight='700' font-size='38' fill='${letter}'>Q</text>
${[[9, 9], [55, 9], [55, 55], [9, 55]].map(([x, y], i) => `<circle cx='${x}' cy='${y}' r='3.6' fill='${dots[i]}'/>`).join('')}
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
