import { createMemo, createSignal, createUniqueId, For, Index, onCleanup, Show } from 'solid-js';
import type { Grid } from '../core/grid';
import type { Frame } from '../core/locale';

interface ClockProps {
  grid: Grid;
  frame: Frame;
  label: string;
  showDots: boolean;
  /** Seconds into the current minute; draws the progress frame when set. */
  progress?: number;
  /** Small line of text in the top margin, e.g. weekday and date. */
  caption?: string;
  /** Background particles (falling leaves, bubbles …); what they look like is up to the theme. */
  ambient?: boolean;
}

const PARTICLES = 14;

/** Fixed pseudo-random spread so particles don't move in lockstep; the theme picks shape, colour and motion. */
const particles = Array.from({ length: PARTICLES }, (_, i) => ({
  '--x': `${(i * 37 + 11) % 100}%`,
  '--d': `${-((i * 1.73) % PARTICLES)}s`,
  '--t': 0.75 + ((i * 7) % 5) * 0.12,
  '--s': 0.7 + ((i * 3) % 4) * 0.2,
  '--r': `${(i * 53) % 360}deg`,
  '--w': ((i * 5) % 7) - 3,
  '--c': `var(--amb-c${(i % 3) + 1})`,
}));

/**
 * Line through the corner dots, filled clockwise from the top centre; pathLength makes one unit one second.
 * Drawn in real pixels: a stretched viewBox would need non-scaling-stroke, which breaks pathLength dashes.
 */
function Progress(props: { seconds: number }) {
  const [size, setSize] = createSignal({ w: 0, h: 0 });
  const observe = (el: SVGSVGElement) => {
    const observer = new ResizeObserver(([entry]) => {
      const box = entry!.contentRect;
      setSize({ w: box.width, h: box.height });
    });
    observer.observe(el);
    onCleanup(() => observer.disconnect());
  };
  // The line fades in over its first stretch instead of starting abruptly at the top centre.
  const id = createUniqueId();
  const fade = () => size().w * 0.25;
  const path = () => {
    const { w, h } = size();
    return `M${w / 2} 0 H${w} V${h} H0 V0 Z`;
  };

  return (
    <svg
      ref={observe}
      class="progress"
      classList={{ wrap: props.seconds === 0 }}
      viewBox={`0 0 ${size().w} ${size().h}`}
      aria-hidden="true"
    >
      <defs>
        <linearGradient id={`${id}-fade`} gradientUnits="userSpaceOnUse" x1={size().w / 2} x2={size().w / 2 + fade()}>
          <stop offset="0" stop-color="#000" />
          <stop offset="1" stop-color="#fff" />
        </linearGradient>
        <mask id={`${id}-mask`} maskUnits="userSpaceOnUse" x="-10" y="-10" width={size().w + 20} height={size().h + 20}>
          <rect x="-10" y="-10" width={size().w + 20} height={size().h + 20} fill="#fff" />
          <rect x={size().w / 2} y="-10" width={fade()} height="20" fill={`url(#${id}-fade)`} />
        </mask>
      </defs>
      <g mask={`url(#${id}-mask)`}>
        <path class="progress-track" d={path()} pathLength="60" />
        <path class="progress-bar" d={path()} pathLength="60" style={{ 'stroke-dashoffset': 60 - props.seconds }} />
      </g>
    </svg>
  );
}

export function Clock(props: ClockProps) {
  // Cell → position in reading order, used to stagger the light-up.
  const order = createMemo(() => {
    const map = new Map<number, number>();
    props.frame.words.flat().forEach((cell, i) => map.has(cell) || map.set(cell, i));
    return map;
  });

  return (
    <div
      class="clock"
      role="img"
      aria-label={props.label}
      style={{ '--rows': props.grid.rows, '--cols': props.grid.cols }}
    >
      <Show when={props.ambient}>
        <div class="ambient" aria-hidden="true">
          <For each={particles}>{(style) => <i style={style} />}</For>
        </div>
      </Show>
      <Show when={props.progress !== undefined}>
        <Progress seconds={props.progress ?? 0} />
      </Show>
      <Show when={props.caption}>
        <div class="caption">{props.caption}</div>
      </Show>
      <Show when={props.showDots}>
        <For each={[1, 2, 3, 4]}>
          {(n) => <span class="dot" data-corner={n} classList={{ on: props.frame.dots >= n }} />}
        </For>
      </Show>
      <div class="chars">
        <Index each={props.grid.chars}>
          {(char, i) => {
            const position = () => order().get(i);
            return (
              <span class="char" classList={{ on: position() !== undefined }} style={{ '--order': position() ?? 0 }}>
                {char()}
              </span>
            );
          }}
        </Index>
      </div>
    </div>
  );
}
